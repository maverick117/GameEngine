use gfx;
use gfx_window_glutin;
use glutin;

use std::sync::mpsc::*;
use super::System;
use super::Msg;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct RenderSystem{
    title: String,
    width: i32,
    height: i32,
    // model_list: Vec<>
}

impl System for RenderSystem {
    fn init(&self) {}
    fn main_loop(&mut self) {}
    fn add_tx(&mut self, msg_tx: Sender<Msg>) {}
    fn set_rx(&mut self, msg_tx: Receiver<Msg>) {}
}

impl RenderSystem {
    pub fn new(title: String, width:i32, height: i32) -> RenderSystem {
        RenderSystem {title, width, height}
    }
}

pub fn init() {
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