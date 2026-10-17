mod app;
mod camera;
mod render;
mod state;
mod voxel;

use app::App;
use winit::event_loop::EventLoop;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().expect("Не удалось создать event loop");
    let mut app = App::new();

    event_loop.run_app(&mut app).expect("Ошибка event loop");
}