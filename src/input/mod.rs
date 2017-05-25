
use std::sync::mpsc::*;

use super::System;
use super::Msg;

pub struct InputSystem {
    //sources: Vec<InputSource>,
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
}

impl System for InputSystem {
    fn init(&self) {}
    fn main_loop(&mut self) {
        loop {
            println!("Running!");
        }
    }
}

impl InputSystem {
    pub fn new(msg_rx: Receiver<Msg>) -> InputSystem {
        InputSystem {
            sources: Vec::new(),
            msg_tx: Vec::new(),
            msg_rx: msg_rx,
        }
    }

    pub fn push_event(input_event: InputEvent) {}
}
