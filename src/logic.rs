/* Logic Module */

use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use glutin::WindowEvent;

use super::System;
use super::Msg;
use super::*;

pub struct LogicSystem {
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
    mouse_x: i32,
    mouse_y: i32,
    // object_list: Vec<Object>,
}

impl System for LogicSystem {
    fn init(&mut self) {}
    fn main_loop(&mut self) {
        let mut should_run = true;
        while should_run {
            let mut cmd_queue = Vec::new();
            while let Ok(msg) = self.msg_rx.try_recv() {
                cmd_queue.push(msg);
            }
            for m in cmd_queue {
                self.msg_tx[2]
                    .send(Msg {
                              content: MsgContent::Debug(format!("Logic System received {:?}", m)),
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
    }
}

impl LogicSystem {
    pub fn new(msg_tx: Vec<Sender<Msg>>, msg_rx: Receiver<Msg>) -> LogicSystem {
        LogicSystem {
            msg_tx: msg_tx,
            msg_rx: msg_rx,
            mouse_x: 0,
            mouse_y: 0,
        }
    }

    fn process_keydown(&mut self, key: glutin::VirtualKeyCode) {}

    fn process_keyup(&mut self, key: glutin::VirtualKeyCode) {}

    fn process_mouseup(&mut self, key: glutin::MouseButton) {}

    fn process_mousedown(&mut self, key: glutin::MouseButton) {}

    fn process_mousemove(&mut self, xcoord: i32, ycoord: i32) {
        let dx = xcoord - self.mouse_x;
        let dy = ycoord - self.mouse_y;
    }
}
