use glam::{Mat4, Vec3};

#[derive(Default)]
pub struct Movement {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub sprint: bool,
}

pub struct Camera {
    position: Vec3,
    yaw: f32,
    pitch: f32,
    aspect: f32,
    pub movement: Movement,
}

impl Camera {
    const BASE_SPEED: f32 = 10.0;
    const SPRINT_MULTIPLIER: f32 = 3.0;
    const MOUSE_SENSITIVITY: f32 = 0.002;
    const MAX_PITCH: f32 = 89.0_f32 * std::f32::consts::PI / 180.0;
    const FOV_DEGREES: f32 = 70.0;
    const NEAR: f32 = 0.1;
    const FAR: f32 = 1000.0;

    pub fn new(position: Vec3, aspect: f32) -> Self {
        Self {
            position,
            yaw: -90.0_f32.to_radians(),
            pitch: 0.0,
            aspect: aspect.max(0.0001),
            movement: Movement::default(),
        }
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect.max(0.0001);
    }

    /// Возвращает текущую позицию камеры (полезно для рейкастов/физики).
    #[allow(dead_code)]
    pub fn eye(&self) -> Vec3 {
        self.position
    }

    pub fn rotate(&mut self, dx: f32, dy: f32) {
        self.yaw += dx * Self::MOUSE_SENSITIVITY;
        self.pitch -= dy * Self::MOUSE_SENSITIVITY;
        self.pitch = self.pitch.clamp(-Self::MAX_PITCH, Self::MAX_PITCH);
    }

    pub fn update(&mut self, dt: f32) {
        let speed = Self::BASE_SPEED
            * if self.movement.sprint {
                Self::SPRINT_MULTIPLIER
            } else {
                1.0
            }
            * dt.max(0.0);

        let forward = self.forward_vector();
        let right = forward.cross(Vec3::Y).normalize_or_zero();

        let mut dir = Vec3::ZERO;
        if self.movement.forward {
            dir += forward;
        }
        if self.movement.backward {
            dir -= forward;
        }
        if self.movement.right {
            dir += right;
        }
        if self.movement.left {
            dir -= right;
        }
        if self.movement.up {
            dir += Vec3::Y;
        }
        if self.movement.down {
            dir -= Vec3::Y;
        }

        // Нормализация, чтобы диагональ не была быстрее прямой.
        if dir.length_squared() > f32::EPSILON {
            self.position += dir.normalize() * speed;
        }
    }

    fn forward_vector(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize_or_zero()
    }

    pub fn view_projection(&self) -> Mat4 {
        let view = Mat4::look_at_rh(
            self.position,
            self.position + self.forward_vector(),
            Vec3::Y,
        );
        let proj = Mat4::perspective_rh(
            Self::FOV_DEGREES.to_radians(),
            self.aspect,
            Self::NEAR,
            Self::FAR,
        );
        proj * view
    }
}