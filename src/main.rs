#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;


mod console;
mod render;
mod input;
mod model;
mod logic;

use std::thread;

use std::sync::mpsc;
use std::sync::mpsc::*;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;

use std::cell::UnsafeCell;

use std::ops::Deref;

use gfx::Device;

use input::*;
use console::*;
use render::*;
use model::*;
use logic::*;

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
                                        vec![render_tx, model_tx, logic_tx],
                                        input_rx);
    // Initialize console system
    let console_system = ConsoleSystem::new(Vec::new(), console_rx);

    // Initialize render system
    // let render_system = RenderSystem::new(...);

    // Initialize model system
    // let model_system = ModelSystem::new(...);

    // Initialize logic system
    // let logic_system = LogicSystem::new(...);



    // Spawn threads for each system
    let input_handle = thread::spawn(move || spawn_systems(input_system));
    let console_handle = thread::spawn(move || spawn_systems(console_system));
    // let render_handle = thread::spawn(move || spawn_systems(render_system));
    // let model_handle = thread::spawn(move || spawn_systems(model_system));
    // let logic_handle = thread::spawn(move || spawn_systems(logic_system));

    // Join threads on exit
    input_handle.join().unwrap();
    console_handle.join().unwrap();
    // render_handle.join().unwrap();
    // model_handle.join().unwrap();
    // logic_handle.join().unwrap();

    println!("Program exited.")

}
