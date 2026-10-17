use super::Vertex;

pub fn create_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    shader: &wgpu::ShaderModule,
) -> wgpu::RenderPipeline {
    // wgpu 30: push_constant_ranges переименовано в immediate_size.
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Voxel Pipeline Layout"),
        bind_group_layouts: &[],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Voxel Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers: &[Some(Vertex::desc())],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        // wgpu 30: multiview переименовано в multiview_mask.
        multiview_mask: None,
        cache: None,
    })
}