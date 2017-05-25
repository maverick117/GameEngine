use gfx;
use gfx_window_glutin;
use glutin;

use std::sync::mpsc::*;
use super::System;
use super::Msg;
use std::sync::Arc;
use std::sync::Mutex;


pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct RenderSystem{
    title: String,
    width: i32,
    height: i32,
    // model_list: Vec<>
    events_loop: Arc<Mutex<glutin::EventsLoop>>,
    // builder: glutin::WindowBuilder,
    // encoder: gfx::Encoder<i32, i32 > ,
    // window: Arc<Mutex<glutin::Window>>,
    // device: gfx::Device,
    // factory: gfx::Factory,
    // main_color: gfx::handle::RenderTargetView<glutin::VirtualKeyCode::R, RenderFormat>,
    // main_depth: gfx::handle::DepthStencilView<glutin::VirtualKeyCode::R, DepthFormat>
    // main_color: i32,
    // main_depth: i32
}

impl System for RenderSystem {
    fn init(&mut self) {
        // self.events_loop = glutin::EventsLoop::new();
        // self.builder = glutin::WindowBuilder::new()
        //     .with_title("Game window".to_string())
        //     .with_dimensions(1024, 768)
        //     .with_vsync();
        // // (self.window, self.device, self.factory, self.main_color, self.main_depth) =
        // //     gfx_window_glutin::init::<ColorFormat, DepthFormat>(self.builder, &self.events_loop);
        // // self.encoder = self.factory.create_command_buffer().into();
        // let (window, device, factory, main_color, main_depth) =
        //     gfx_window_glutin::init::<ColorFormat, DepthFormat>(self.builder, &self.events_loop);
        // let encoder = factory.create_command_buffer().into();

        // main_color = main_color + 1;
        // main_depth = main_depth + 1;
        // encoder = encoder + 1;

        
    }
    fn main_loop(&mut self) {
        // let mut running = true;
        // while running {
        //     // fetch events
            

        //     // draw a frame
        //     self.encoder.flush(&mut self.device);
        //     self.window.swap_buffers().unwrap();
        // }
    }
}

impl RenderSystem {
    pub fn new(title: String, width:i32, height: i32, events_loop: Arc<Mutex<glutin::EventsLoop>>) -> RenderSystem {
        RenderSystem {
            title: title, 
            width: width,
            height: height,
            events_loop: events_loop,
        }
    }
}