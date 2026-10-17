use glam::{Mat4, Vec3};

#[derive(Default)]
pub struct Movement {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

pub struct Camera {
    position: Vec3,
    yaw: f32,
    pitch: f32,
    aspect: f32,
    pub movement: Movement,
}

impl Camera {
    pub fn new(position: Vec3, aspect: f32) -> Self {
        Self {
            position,
            yaw: -90.0_f32.to_radians(),
            pitch: 0.0,
            aspect,
            movement: Movement::default(),
        }
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn eye(&self) -> Vec3 {
        self.position
    }

    // Вращение камеры мышью
    pub fn rotate(&mut self, dx: f32, dy: f32) {
        let sensitivity = 0.002;
        self.yaw += dx * sensitivity;
        self.pitch -= dy * sensitivity;

        // Ограничиваем наклон вверх/вниз, чтобы не переворачивалось
        let max_pitch = 89.0_f32.to_radians();
        self.pitch = self.pitch.clamp(-max_pitch, max_pitch);
    }

    pub fn update(&mut self, dt: f32) {
        let speed = 16.0 * dt; // Ускорили движение для полёта
        let forward = self.forward_vector();
        let right = forward.cross(Vec3::Y).normalize();

        if self.movement.forward { self.position += forward * speed; }
        if self.movement.backward { self.position -= forward * speed; }
        if self.movement.left { self.position -= right * speed; }
        if self.movement.right { self.position += right * speed; }
        if self.movement.up { self.position += Vec3::Y * speed; }
        if self.movement.down { self.position -= Vec3::Y * speed; }
    }

    fn forward_vector(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize()
    }

    pub fn view_projection(&self) -> Mat4 {
        let view = Mat4::look_at_rh(
            self.position,
            self.position + self.forward_vector(),
            Vec3::Y,
        );
        let proj = Mat4::perspective_rh(70.0_f32.to_radians(), self.aspect, 0.1, 1000.0);
        proj * view
    }
}