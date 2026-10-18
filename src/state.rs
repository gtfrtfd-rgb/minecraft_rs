use crate::camera::Camera;
use crate::render::Renderer;
use crate::voxel::World;
use std::sync::Arc;
use std::time::Instant;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window};

pub struct State {
    window: Arc<Window>,
    renderer: Renderer,
    camera: Camera,
    world: World,
    last_frame: Instant,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();

        let renderer = Renderer::new(window.clone(), size).await;

        // Курсор прячем и захватываем. На Wayland `Locked` может не работать —
        // пробуем `Confined` как fallback.
        window.set_cursor_visible(false);
        Self::try_grab_cursor(&window);

        let camera = Camera::new(
            glam::Vec3::new(32.0, 30.0, 32.0),
            size.width as f32 / size.height.max(1) as f32,
        );

        let world = World::new();

        Self {
            window,
            renderer,
            camera,
            world,
            last_frame: Instant::now(),
        }
    }

    fn try_grab_cursor(window: &Window) {
        if let Err(e) = window.set_cursor_grab(CursorGrabMode::Locked) {
            log::warn!("Locked cursor grab failed ({e:?}), trying Confined");
            if let Err(e2) = window.set_cursor_grab(CursorGrabMode::Confined) {
                log::warn!("Confined cursor grab failed ({e2:?})");
            }
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.renderer.resize(size);
            self.camera
                .set_aspect(size.width as f32 / size.height as f32);
        }
    }

    pub fn handle_focus(&self, focused: bool) {
        if focused {
            Self::try_grab_cursor(&self.window);
        }
    }

    pub fn handle_key(&mut self, event: KeyEvent) {
        let pressed = event.state == ElementState::Pressed;

        match event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) => self.camera.movement.forward = pressed,
            PhysicalKey::Code(KeyCode::KeyS) => self.camera.movement.backward = pressed,
            PhysicalKey::Code(KeyCode::KeyA) => self.camera.movement.left = pressed,
            PhysicalKey::Code(KeyCode::KeyD) => self.camera.movement.right = pressed,
            PhysicalKey::Code(KeyCode::Space) => self.camera.movement.up = pressed,
            PhysicalKey::Code(KeyCode::ShiftLeft) => self.camera.movement.down = pressed,
            PhysicalKey::Code(KeyCode::ControlLeft) => self.camera.movement.sprint = pressed,
            _ => {}
        }
    }

    pub fn handle_mouse_motion(&mut self, dx: f32, dy: f32) {
        self.camera.rotate(dx, dy);
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        // Ограничиваем dt, чтобы после tab-out/фриза камера не телепортировалась.
        let dt = (now - self.last_frame).as_secs_f32().min(0.1);
        self.last_frame = now;
        self.camera.update(dt);
    }

    pub fn render(&mut self) {
        let view_proj = self.camera.view_projection();
        self.renderer.render(self.world.mesh(), view_proj);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}