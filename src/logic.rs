/* Logic Module */

use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use glutin::Event;
use tobj;
use tobj::{Model, Material, Mesh};

use super::System;
use super::Msg;
use super::*;
use model::*;
use cgmath;
use render::*;

#[derive(Clone)]
pub enum LogicMsg {
    //Scene(Box<amethyst_renderer::Scene<gfx_device_gl::Resources>>),
    ModelReq(Box<String>),
    SceneSnt(Scene),
}

impl fmt::Debug for LogicMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Zean has come!")
    }
}

pub struct Light {
    position: cgmath::Point3<f32>,
    ambient: cgmath::Vector3<f32>,
    diffuse: cgmath::Vector3<f32>,
    speculer: cgmath::Vector3<f32>,

}

pub struct Object {
    models: Vec<Model>,
}

impl Object {
    pub fn new(models: Vec<Model>) -> Object {
        Object { models: models }
    }

    pub fn add_model(&mut self, models: Vec<Model>) {
        for m in models {
            self.models.push(m);
        }
    }

    fn translate(&mut self, dx: f64, dy: f64, dz: f64) {}
    fn rotate(&mut self, rad: f64) {}
    fn scale(&mut self, s: f64) {}

    fn gen_fragment(&self) {} // TODO: generate fragment
}

pub struct LogicSystem {
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
    mouse_x: i32,
    mouse_y: i32,
    object_list: Vec<Object>,
    selected_object_index: usize,
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

            // For all objects, continue on their trajectories

            // Generate fragments

            // Pass to renderer
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
            object_list: Vec::new(),
            selected_object_index: 0,
        }
    }

    fn process_keydown(&mut self, key: glutin::VirtualKeyCode) {
        // Logic for key down
        use glutin::VirtualKeyCode::*;
        match key {
            Up => {
                self.object_list[self.selected_object_index].translate(0.0, 0.1, 0.0);
            }
            Down => {
                self.object_list[self.selected_object_index].translate(0.0, -0.1, 0.0);
            }
            Left => {
                self.object_list[self.selected_object_index].translate(0.1, 0.0, 0.0);
            }
            Right => {
                self.object_list[self.selected_object_index].translate(-0.1, 0.0, 0.0);
            }
            Space => {
                // Generate another projectile
            }
            Snapshot => {}
            _ => {}
        }
    }

    fn process_keyup(&mut self, key: glutin::VirtualKeyCode) {}

    fn process_mouseup(&mut self, key: glutin::MouseButton) {}

    fn process_mousedown(&mut self, key: glutin::MouseButton) {}

    fn process_mousemove(&mut self, xcoord: i32, ycoord: i32) {
        let dx = xcoord - self.mouse_x;
        let dy = ycoord - self.mouse_y;
    }
}
