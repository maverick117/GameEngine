
use std::sync::mpsc::*;

use super::System;
use super::Msg;
use super::*;

pub struct InputSystem {
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
}

impl System for InputSystem {
    fn init(&mut self) {}
    fn main_loop(&mut self) {
        loop {
            for tx in ref self.msg_tx {
                tx.send(Msg { content: MsgContent::System(SystemMsg::SysInit) });
            }
        }
    }

    //fn set_rx(&mut self, msg_tx: Receiver<Msg>) {}
}

impl InputSystem {
    pub fn new(msg_tx: Vec<Sender<Msg>>, msg_rx: Receiver<Msg>) -> InputSystem {
        InputSystem {
            //sources: Vec::new(),
            msg_tx: Vec::new(),
            msg_rx: msg_rx,
        }
    }

    //pub fn push_event(input_event: InputEvent) {}
}
