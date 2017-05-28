
use std::sync::mpsc::*;
use std::time::SystemTime;

use super::System;
use super::Msg;
use super::*;

pub struct ConsoleSystem {
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
    startup_time: SystemTime,
}

impl System for ConsoleSystem {
    fn init(&mut self) {
        println!("Console Running.");
        self.startup_time = SystemTime::now();
    }

    fn main_loop(&mut self) {
        loop {
            let msg = self.msg_rx.recv().unwrap();
            let elapsed_time = SystemTime::now()
                .duration_since(self.startup_time)
                .expect("duration_since failed.");
            //println!("{:?}: {:?}", elapsed_time, msg);
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
            startup_time: SystemTime::now(),
        }
    }
}
