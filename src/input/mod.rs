/* Input module */

use glutin;
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use glutin::WindowEvent;

use super::System;
use super::Msg;
use super::*;

pub struct InputSystem {
    events_loop: Arc<Mutex<glutin::EventsLoop>>,
    window: Arc<Mutex<glutin::Window>>,
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
}

impl InputSystem {
    pub fn new(events_loop: Arc<Mutex<glutin::EventsLoop>>,
               window: Arc<Mutex<glutin::Window>>,
               msg_tx: Vec<Sender<Msg>>,
               msg_rx: Receiver<Msg>)
               -> InputSystem {

        InputSystem {
            events_loop: events_loop,
            window: window,
            msg_tx: msg_tx,
            msg_rx: msg_rx,
        }

    }
}


impl System for InputSystem {
    fn init(&mut self) {
        let init_msg = Msg { content: MsgContent::System(SystemMsg::SysInit) };
        for tx in &self.msg_tx {
            tx.send(init_msg.clone());
        }
    }

    fn main_loop(&mut self) {
        let mut should_run = true;
        while should_run {
            use glutin::WindowEvent::*;
            use glutin::VirtualKeyCode;
            self.events_loop
                .lock()
                .unwrap()
                .poll_events(|glutin::Event::WindowEvent { window_id, event }| {
                    let debug_msg = Msg { content: MsgContent::Debug(format!("{:?}", event)) };
                    self.msg_tx[3].send(debug_msg);
                    match event {
                        Resized(width, height) => {
                            let resize_msg =
                                Msg { content: MsgContent::Input(InputMsg::Resize(width, height)) };
                            self.msg_tx[0].send(resize_msg);
                        }
                        Closed => {
                            use super::{MsgContent, SystemMsg};
                            let halt_msg = Msg { content: MsgContent::System(SystemMsg::SysHalt) };
                            for tx in &self.msg_tx {
                                tx.send(halt_msg.clone()).unwrap();
                            }
                            should_run = false;
                        }

                        _ => {}
                    }
                });
        }
    }

    //fn set_rx(&mut self, msg_tx: Receiver<Msg>) {}
}
