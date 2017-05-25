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
    fn init(&self);
    fn main_loop(&mut self);
    fn add_tx(&mut self, msg_tx: Sender<Msg>);
    //fn set_rx(&mut self, msg_rx: Receiver<Msg>);
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


    let input_system: InputSystem = InputSystem::new();

    let (input_tx, input_rx) = mpsc::channel();
    
    let input_handle = thread::spawn(move || spawn_systems(input_system));

    input_handle.join().unwrap();


}
