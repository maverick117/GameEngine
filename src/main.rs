mod console;
mod render;
mod input;
mod model;

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct Msg {

}

pub trait System{
    fn handle_message(msg: Msg);


}

pub fn main() {
    let events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Game window".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut running = true;
    while running {
        // fetch events
        

        // draw a frame
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
    }
}