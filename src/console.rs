
use std::sync::mpsc::*;

use super::System;
use super::Msg;

pub struct ConsoleSystem {
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
}

impl System for ConsoleSystem {
    fn init(&mut self) {
        println!("Console Running.");
    }

    fn main_loop(&mut self) {
        println!("{:?}", self.msg_rx.recv().unwrap());
    }
}

impl ConsoleSystem {
    pub fn new(msg_tx: Vec<Sender<Msg>>, msg_rx: Receiver<Msg>) -> ConsoleSystem {
        ConsoleSystem {
            msg_tx: msg_tx,
            msg_rx: msg_rx,
        }
    }
}
