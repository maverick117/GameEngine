mod logic_source;

use std::sync::mpsc::*;

use self::logic_source::*;
use super::System;
use super::Msg;

pub struct LogicSystem {
    sources: Vec<InputSource>,
    //msg_tx: Sender<Msg>,
    //msg_rx: Receiver<Msg>,
}

impl System for LogicSystem {
    fn init(&self) {}
    fn main_loop(&mut self) {
        loop {
            println!("Logic System Running!");
        }
    }
    fn add_tx(&mut self, msg_tx: Sender<Msg>) {}
    fn set_rx(&mut self, msg_tx: Receiver<Msg>) {}
}

impl LogicSystem {
    pub fn new() -> LogicSystem {
        LogicSystem { sources: Vec::new() }
    }
}
