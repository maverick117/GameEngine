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
use cgmath::*;
use render::*;
use tool::*;

#[derive(Clone)]
pub enum LogicMsg {
    ModelReq(String),
    SceneSnd(Scene),
}

impl fmt::Debug for LogicMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Zean has come!")
    }
}

#[derive(Clone, Debug)]
pub struct Light {
    position: cgmath::Point3<f32>,
    ambient: cgmath::Vector3<f32>,
    diffuse: cgmath::Vector3<f32>,
    speculer: cgmath::Vector3<f32>,
}

#[derive(Debug)]
enum Axis {
    Axis_x,
    Axis_y,
    Axis_z,
    Axis_any(Vector3<f32>),
}

#[derive(Clone, Debug)]
pub struct Object {
    pub models: Vec<Model>,
    pub materials: Vec<Material>,
    translate_matrix: Matrix4<f32>,
    rotate_matrix: Matrix4<f32>,
    scale_matrix: Matrix4<f32>,
    // model_transform_matrix: [[f32; 4]; 4],
    pub path: String,
}

impl Object {
    pub fn new(models: Vec<Model>, materials: Vec<Material>, path: String) -> Object {
        Object {
            models: models,
            materials: materials,
            translate_matrix: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
            rotate_matrix: Matrix4::from_angle_x(Rad(0.0)),
            scale_matrix: Matrix4::from_scale(1.0),
            path: path,
        }
    }

    fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        self.translate_matrix = Matrix4::from_translation(Vector3::new(dx, dy, dz)) *
                                self.translate_matrix;
    }
    fn rotate(&mut self, axis: Axis, angle: f32) {
        let rot = match axis {
            Axis::Axis_x => Matrix4::from_angle_x(Deg(angle)),
            Axis::Axis_y => Matrix4::from_angle_y(Deg(angle)),
            Axis::Axis_z => Matrix4::from_angle_z(Deg(angle)),
            Axis::Axis_any(v) => Matrix4::from_axis_angle(v, Deg(angle)),
        };
        self.rotate_matrix = rot * self.rotate_matrix;
    }
    fn scale(&mut self, ratio: f32) {
        self.scale_matrix = Matrix4::from_scale(ratio) * self.scale_matrix;
    }
    pub fn get_model_matrix(&self) -> [[f32; 4]; 4] {
        let result = self.translate_matrix * self.scale_matrix * self.rotate_matrix;
        // self.model_transform_matrix = result.getArray();
        result.getArray()
    }
}

pub struct LogicSystem {
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
    mouse_x: i32,
    mouse_y: i32,
    object_list: Vec<Object>,
    selected_object_index: usize,
    scene: Scene,
}

impl System for LogicSystem {
    fn init(&mut self) {
        use MsgContent::*;
        use model::ModelMsg::*;
        let light = Light {
                position: cgmath::Point3::new(0.0, 1.0, 0.0),
                ambient: cgmath::Vector3::new(1.0, 1.0, 1.0),
                diffuse: cgmath::Vector3::new(1.0, 1.0, 1.0),
                speculer: cgmath::Vector3::new(1.0, 1.0, 1.0),
        };
        let mut static_object_path: Vec<String> = Vec::new();
        static_object_path.push("./assets/cube.obj".to_string());

        for path in static_object_path {
            let msg = Msg { content: Logic(LogicMsg::ModelReq(path.clone()))};
            self.msg_tx[2].send(msg);
            if let Ok(msg) = self.msg_rx.recv() {
                match msg.content {
                    Model(ObjectResult(Some(obj))) => {
                        self.scene.objects.push(obj);
                    },
                    Model(ObjectResult(None)) => unimplemented!(),
                    _ => unimplemented!(),
                }
            }
        }
        let perspective = cgmath::Perspective{
            left: -2.0,
            right: 2.0,
            bottom: -2.0,
            top: 2.0,
            near: 0.0,
            far: 10.0,
            };
        self.scene.camera = Camera::new(Point3::new(0.0, 0.0, 1.0), Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        self.scene.camera.set_projection_matrix(cgmath::Matrix4::from(perspective));
        
    }
    fn main_loop(&mut self) {
        use MsgContent::*;
        let mut should_run = true;
        while should_run {
            let mut cmd_queue = Vec::new();
            cmd_queue.push(self.msg_rx.recv().unwrap());
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
            // Pass to renderer
            let msg = Msg { content: Logic(LogicMsg::SceneSnd(self.scene.clone()))};
            self.msg_tx[1].send(msg);
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
            scene: Scene::new(Vec::new(), Vec::new(), Camera::new(Point3::new(0.0, 0.0, 1.0), Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0))),
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
