
use baal;
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use glutin::WindowEvent;

use super::System;
use super::Msg;
use super::*;
use model::*;

pub struct AudioSystem {
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
    setting: baal::Setting,
}

impl System for AudioSystem {
    fn init(&mut self) {
        baal::init(&self.setting).unwrap();
        baal::music::play(0);
        baal::effect::set_listener([1., 1., 1.]);

    }
    fn main_loop(&mut self) {
        let mut should_run = true;
        while should_run {
            let mut cmd_queue = Vec::new();
            while let Ok(msg) = self.msg_rx.try_recv() {
                cmd_queue.push(msg);
            }
            for m in cmd_queue {
                self.msg_tx[1]
                    .send(Msg {
                              content: MsgContent::Debug(format!("Audio System received {:?}", m)),
                          })
                    .unwrap();
                match m.content {
                    MsgContent::System(SystemMsg::SysHalt) => {
                        should_run = false;
                    }
                    c => {}
                }
            }
        }
        baal::close();
    }
}

impl AudioSystem {
    pub fn new(msg_tx: Vec<Sender<Msg>>,
               msg_rx: Receiver<Msg>,
               setting: baal::Setting)
               -> AudioSystem {
        AudioSystem {
            msg_tx: msg_tx,
            msg_rx: msg_rx,
            setting: setting,
        }
    }
}
