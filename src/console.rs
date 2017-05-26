
use std::sync::mpsc::*;

use super::System;
use super::Msg;
use super::*;

pub struct ConsoleSystem {
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
}

impl System for ConsoleSystem {
    fn init(&mut self) {
        println!("Console Running.");
    }

    fn main_loop(&mut self) {
        loop {
            let msg = self.msg_rx.recv().unwrap();
            println!("Console Msg Received: {:?}", msg);
            if let MsgContent::System(SystemMsg::SysHalt) = msg.content {
                break;
            }
        }
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
