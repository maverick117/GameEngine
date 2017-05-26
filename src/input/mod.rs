/* Input module */

use glium;
use glium::glutin;
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use glium::glutin::Event;
use glium::backend::glutin_backend::GlutinFacade;


use super::System;
use super::Msg;
use super::*;

#[derive(Copy,Clone,Debug)]
pub enum InputMsg {
    KeyDown(glutin::VirtualKeyCode),
    KeyUp(glutin::VirtualKeyCode),
    MouseMoved(i32, i32),
    MouseDown(glutin::MouseButton),
    MouseUp(glutin::MouseButton),
    Resize(u32, u32),
    Moved(i32, i32),
}

pub struct InputSystem {
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
}

impl InputSystem {
    pub fn new(msg_tx: Vec<Sender<Msg>>, msg_rx: Receiver<Msg>) -> InputSystem {
        InputSystem {
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
            use glutin::Event::*;
            use glutin::VirtualKeyCode;
            if let MsgContent::Render(RenderMsg::InputQueue(queue)) =
                self.msg_rx.recv().unwrap().content {
                for event in queue {
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
                        KeyboardInput(state, ch, Some(key)) => {
                            match state {
                                glutin::ElementState::Pressed => {
                                    let pressed_msg =
                                        Msg { content: MsgContent::Input(InputMsg::KeyDown(key)) };
                                    self.msg_tx[2].send(pressed_msg);
                                }
                                glutin::ElementState::Released => {
                                    let lifted_msg =
                                        Msg { content: MsgContent::Input(InputMsg::KeyUp(key)) };
                                    self.msg_tx[2].send(lifted_msg);
                                }
                            };
                        }
                        MouseMoved(xcoord, ycoord) => {
                            let mouse_move_msg = Msg {
                                content: MsgContent::Input(InputMsg::MouseMoved(xcoord, ycoord)),
                            };
                            self.msg_tx[2].send(mouse_move_msg);
                        }

                        _ => {}
                    }
                }
            }
        }
    }

    //fn set_rx(&mut self, msg_tx: Receiver<Msg>) {}
}
