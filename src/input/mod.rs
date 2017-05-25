mod input_source;

use std::sync::mpsc::*;

use self::input_source::*;
use super::System;
use super::Msg;

pub struct InputSystem {
    sources: Vec<InputSource>,
    //msg_tx: Sender<Msg>,
    //msg_rx: Receiver<Msg>,
}

impl System for InputSystem {
    fn init(&self) {}
    fn main_loop(&mut self) {
        loop {
            println!("Running!");
        }
    }
    fn add_tx(&mut self, msg_tx: Sender<Msg>) {}
    fn set_rx(&mut self, msg_tx: Receiver<Msg>) {}
}

impl InputSystem {
    pub fn new() -> InputSystem {
        InputSystem { sources: Vec::new() }
    }
}
