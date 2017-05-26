#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;


mod console;
//mod render;
mod input;
mod model;

use std::sync::mpsc;
use std::sync::mpsc::*;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::cell::UnsafeCell;
// use input::InputSystem;
//use render::RenderSystem;
use console::ConsoleSystem;
use gfx::Device;
use std::ops::Deref;

use input::*;

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
    KeyDown(glutin::VirtualKeyCode),
    KeyUp(glutin::VirtualKeyCode),
    MouseMoved(i32, i32),
    MouseDown(glutin::MouseButton),
    MouseUp(glutin::MouseButton),
    Resize(u32, u32),
    Moved(i32, i32),
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

#[derive(Clone,Debug)]
pub enum MsgContent {
    System(SystemMsg),
    Input(InputMsg),
    Render(RenderMsg),
    Model(ModelMsg),
    Logic(LogicMsg),
    Debug(String),
}

#[derive(Clone,Debug)]
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

    // Create tunnels for message passing
    let (input_tx, input_rx) = mpsc::channel();
    let (render_tx, render_rx) = mpsc::channel();
    let (model_tx, model_rx) = mpsc::channel();
    let (logic_tx, logic_rx) = mpsc::channel();
    let (console_tx, console_rx) = mpsc::channel();

    let events_loop = Arc::new(Mutex::new(glutin::EventsLoop::new()));
    let builder = glutin::WindowBuilder::new()
        .with_title("Engine Demo".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder,
                                                            events_loop.lock().unwrap().deref());

    let arc_window = Arc::new(Mutex::new(window));

    // Initialize input system
    let input_system = InputSystem::new(events_loop.clone(),
                                        arc_window.clone(),
                                        vec![render_tx, model_tx, logic_tx, console_tx],
                                        input_rx);
    let console_system = ConsoleSystem::new(Vec::new(), console_rx);






    let input_handle = thread::spawn(move || spawn_systems(input_system));
    let console_handle = thread::spawn(move || spawn_systems(console_system));

    input_handle.join().unwrap();
    console_handle.join().unwrap();


}
