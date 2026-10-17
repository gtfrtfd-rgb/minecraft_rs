use crate::camera::Camera;
use crate::render::Renderer;
use crate::voxel::World;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::keyboard::KeyCode;
use winit::window::Window;

pub struct State {
    window: Arc<Window>,
    renderer: Renderer,
    camera: Camera,
    world: World,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();

        let renderer = Renderer::new(window.clone(), size).await;

        // Камера парит над миром
        let camera = Camera::new(
            glam::Vec3::new(32.0, 30.0, 32.0),
            size.width as f32 / size.height as f32,
        );

        let world = World::new();

        Self {
            window,
            renderer,
            camera,
            world,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.renderer.resize(size);
            self.camera
                .set_aspect(size.width as f32 / size.height as f32);
        }
    }

    pub fn handle_key(&mut self, event: winit::event::KeyEvent) {
        use winit::event::ElementState;
        if event.state == ElementState::Pressed {
            match event.physical_key {
                winit::keyboard::PhysicalKey::Code(KeyCode::Escape) => {
                    std::process::exit(0);
                }
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyW) => {
                    self.camera.movement.forward = true;
                }
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyS) => {
                    self.camera.movement.backward = true;
                }
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyA) => {
                    self.camera.movement.left = true;
                }
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyD) => {
                    self.camera.movement.right = true;
                }
                winit::keyboard::PhysicalKey::Code(KeyCode::Space) => {
                    self.camera.movement.up = true;
                }
                winit::keyboard::PhysicalKey::Code(KeyCode::ShiftLeft) => {
                    self.camera.movement.down = true;
                }
                _ => {}
            }
        } else {
            match event.physical_key {
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyW) => {
                    self.camera.movement.forward = false;
                }
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyS) => {
                    self.camera.movement.backward = false;
                }
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyA) => {
                    self.camera.movement.left = false;
                }
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyD) => {
                    self.camera.movement.right = false;
                }
                winit::keyboard::PhysicalKey::Code(KeyCode::Space) => {
                    self.camera.movement.up = false;
                }
                winit::keyboard::PhysicalKey::Code(KeyCode::ShiftLeft) => {
                    self.camera.movement.down = false;
                }
                _ => {}
            }
        }
    }

    pub fn handle_mouse_motion(&mut self, dx: f32, dy: f32) {
        self.camera.rotate(dx, dy);
    }

    pub fn update(&mut self) {
        self.camera.update(0.016);
    }

    pub fn render(&mut self) {
        let view_proj = self.camera.view_projection();
        let mesh_data = self.world.generate_mesh_data();

        self.renderer
            .render(&mesh_data, view_proj, &self.camera.eye());
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}