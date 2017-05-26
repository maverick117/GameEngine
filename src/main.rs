#[macro_use]
extern crate glium;
extern crate baal;

mod console;
mod render;
//mod logic;
mod input;
//mod model;
mod audio;

use std::sync::mpsc;
use std::sync::mpsc::*;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::cell::UnsafeCell;
use std::fmt;
use input::InputSystem;
use render::*;
use console::ConsoleSystem;

use std::ops::Deref;
use audio::*;
use input::*;
//use logic::LogicSystem;
use glium::glutin::Event;
use glium::glutin;
use glium::DisplayBuild;

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

#[derive(Clone,Debug)]
pub enum RenderMsg {
    InputQueue(Vec<Event>),
}

#[derive(Copy,Clone,Debug)]
pub enum ModelMsg {

}

#[derive(Clone)]
pub enum LogicMsg {
    //Scene(Box<amethyst_renderer::Scene<gfx_device_gl::Resources>>),
    //ModelReq(Box<String>),
}

impl fmt::Debug for LogicMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Zean has come!")
    }
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


fn spawn_systems<T, F>(some_closure: F, msg_tx: Vec<Sender<Msg>>, msg_rx: Receiver<Msg>)
    where T: System,
          F: Fn(Vec<Sender<Msg>>, Receiver<Msg>) -> T
{
    println!("Spawning systems");
    let mut sys = some_closure(msg_tx, msg_rx);
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
    let (audio_tx, audio_rx) = mpsc::channel();
    let (main_tx, main_rx) = mpsc::channel();

    //let console_system = ConsoleSystem::new(Vec::new(), console_rx);

    /*
    let logic_system = LogicSystem::new(vec![render_tx.clone(),
                                             model_tx.clone(),
                                             console_tx.clone(),
                                             audio_tx.clone()],
                                        logic_rx);
                                        */

    let audio_setting = baal::Setting {
        effect_dir: "assets/fx".into(),
        music_dir: "assets/stream".into(),

        global_volume: 1.0,
        music_volume: 1.0,
        effect_volume: 1.0,

        distance_model: baal::effect::DistanceModel::Linear(10., 100.),

        music_transition: baal::music::MusicTransition::Instant,
        short_effects: vec!["wowa-intro.ogg".into()],
        persistent_effects: vec!["wowa-intro.ogg".into()],
        musics: vec!["to_be_free.ogg".into()],
    };


    //let audio_system = AudioSystem::new(vec![logic_tx.clone(), console_tx.clone()],
    //audio_rx,
    //audio_setting);



    let tmp_vec = vec![render_tx.clone(),
                       model_tx.clone(),
                       logic_tx.clone(),
                       console_tx.clone(),
                       audio_tx.clone(),
                       main_tx.clone()];
    let input_handle =
        thread::spawn(move || {
                          spawn_systems(|msg_tx, msg_rx| InputSystem::new(msg_tx, msg_rx),
                                        tmp_vec,
                                        input_rx)
                      });
    let tmp_vec = vec![input_tx.clone(),
                       model_tx.clone(),
                       logic_tx.clone(),
                       console_tx.clone(),
                       audio_tx.clone(),
                       main_tx.clone()];
    let render_handle =
        thread::spawn(move || {
                          spawn_systems(|msg_tx, msg_rx| RenderSystem::new(msg_tx, msg_rx),
                                        tmp_vec,
                                        render_rx)
                      });
    let tmp_vec = vec![input_tx.clone(),
                       render_tx.clone(),
                       model_tx.clone(),
                       logic_tx.clone(),
                       console_tx.clone(),
                       main_tx.clone()];
    let audio_handle =
        thread::spawn(move || {
                          spawn_systems(|msg_tx, msg_rx| AudioSystem::new(msg_tx, msg_rx),
                                        tmp_vec,
                                        audio_rx)
                      });
    //let console_handle = thread::spawn(move || spawn_systems(console_system));
    //let logic_handle = thread::spawn(move || spawn_systems(logic_system));
    //let audio_handle = thread::spawn(move || spawn_systems(audio_system));

    input_handle.join().unwrap();
    render_handle.join().unwrap();
    //console_handle.join().unwrap();
    //logic_handle.join().unwrap();
    audio_handle.join().unwrap();


}
