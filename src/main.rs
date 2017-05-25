mod console;
mod render;
mod input;
mod model;

use std::sync::mpsc::*;
use std::thread;
use std::sync::mpsc::channel;

use input::InputSystem;

pub enum SystemMsg {
    SysInit,
    SysHalt,
    SysUpdate,
    SysFlush,
}

pub enum InputMsg {

}

pub enum RenderMsg {

}

pub enum ModelMsg {

}

pub enum LogicMsg {

}

pub enum MsgContent {
    System(SystemMsg),
    Input(InputMsg),
    Render(RenderMsg),
    Model(ModelMsg),
    Logic(LogicMsg),
}

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

    // Initialize input system
    let input_system = InputSystem::new(vec![input_tx.clone(),
                                             render_tx.clone(),
                                             model_tx.clone(),
                                             logic_tx.clone()],
                                        input_rx);



    let input_handle = thread::spawn(move || spawn_systems(input_system));

    input_handle.join().unwrap();


}
