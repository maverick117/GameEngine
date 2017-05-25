#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate lazy_static;

mod console;
mod render;
// mod input;
mod model;

use std::sync::mpsc;
use std::sync::mpsc::*;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::cell::UnsafeCell;
// use input::InputSystem;
use render::RenderSystem;
use console::ConsoleSystem;
use gfx::Device;
use std::ops::Deref;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;


#[derive(Copy,Clone,Debug)]
pub enum SystemMsg {
    SysInit,
    SysHalt,
    SysUpdate,
    SysFlush,
}


#[derive(Copy,Clone,Debug)]
pub enum InputMsg {

}

#[derive(Copy,Clone,Debug)]
pub enum RenderMsg {

}

#[derive(Copy,Clone,Debug)]
pub enum ModelMsg {

}

#[derive(Copy,Clone,Debug)]
pub enum LogicMsg {
}

#[derive(Copy,Clone,Debug)]
pub enum MsgContent {
    System(SystemMsg),
    Input(InputMsg),
    Render(RenderMsg),
    Model(ModelMsg),
    Logic(LogicMsg),
}

#[derive(Copy,Clone,Debug)]
pub struct Msg {
    content: MsgContent,
    // Other fields
}

pub trait System {
    fn init(&mut self);
    fn main_loop(&mut self);
}


fn spawn_systems<T>(mut sys: T)
    where T: System
{
    println!("Spawning systems");
    sys.init();
    sys.main_loop();
}

fn main() {

    println!("Welcome to Game Engine. Initializing all systems");

    let events_loop = Arc::new(Mutex::new(glutin::EventsLoop::new()));
    //let arc_events_loop = unsafe{UnsafeCell::new(events_loop)};
    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, events_loop.lock().unwrap().deref());
    // let arc_window = Arc::new(Mutex::new(window));
    let arc_window = Arc::new(Mutex::new(window)); 

    let render_engine = RenderSystem::new("test".to_string(), 1024, 768, events_loop.clone());
    let console_engine = ConsoleSystem::new(events_loop.clone());
    let mut running = true;
    while running {
        // fetch events
        // draw a frame
        arc_window.lock().unwrap().swap_buffers().unwrap();
        device.cleanup();
    }


    // Create tunnels for message passing
    // let (input_tx, input_rx) = mpsc::channel();
    // let (render_tx, render_rx) = mpsc::channel();
    // let (model_tx, model_rx) = mpsc::channel();
    // let (logic_tx, logic_rx) = mpsc::channel();

    // Initialize input system
    // let input_system = InputSystem::new(vec![input_tx.clone(),
    //                                          render_tx.clone(),
    //                                          model_tx.clone(),
    //                                          logic_tx.clone()],
    //                                     input_rx);



    // let input_handle = thread::spawn(move || spawn_systems(input_system));

    // input_handle.join().unwrap();



}
