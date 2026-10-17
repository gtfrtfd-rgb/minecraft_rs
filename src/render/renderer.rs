use super::{pipeline, Vertex};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
}

impl Renderer {
    pub async fn new(window: Arc<Window>, size: PhysicalSize<u32>) -> Self {
        let mut instance_desc = wgpu::InstanceDescriptor::new_without_display_handle();
        instance_desc.backends = wgpu::Backends::VULKAN;
        let instance = wgpu::Instance::new(instance_desc);

        let surface = instance
            .create_surface(window.clone())
            .expect("Не удалось создать surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: false,
            })
            .await
            .expect("Vulkan-адаптер не найден");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Vulkan Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .expect("Не удалось создать device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Srgb,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Voxel Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        let pipeline = pipeline::create_pipeline(&device, &config, &shader);

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Voxel Vertex Buffer"),
            size: 1024 * 1024,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            vertex_buffer,
            vertex_count: 0,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.config.width = size.width.max(1);
        self.config.height = size.height.max(1);
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self, vertices: &[Vertex], _view_proj: glam::Mat4, _eye: &glam::Vec3) {
        if vertices.is_empty() {
            return;
        }

        let vertex_data = bytemuck::cast_slice(vertices);
        if vertex_data.len() as u64 > self.vertex_buffer.size() {
            // Исправление: при пересоздании буфера сохраняем COPY_DST,
            // чтобы write_buffer работал в следующих кадрах.
            self.vertex_buffer = self.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Voxel Vertex Buffer"),
                    contents: vertex_data,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                },
            );
        } else {
            self.queue.write_buffer(&self.vertex_buffer, 0, vertex_data);
        }
        self.vertex_count = vertices.len() as u32;

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
            e => {
                log::error!("surface error: {e:?}");
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.45,
                            g: 0.7,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.vertex_count, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        self.queue.present(output);
    }
}