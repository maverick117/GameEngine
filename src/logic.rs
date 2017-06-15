/* Logic Module */

use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
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
        write!(f, "New Scene")
    }
}


#[derive(Clone, Debug)]
pub struct Light {
    pub position: [f32; 4],
    pub color: [f32; 3],
    pub attenuation: [f32; 3],
    pub radius: f32,
}

#[derive(Debug)]
enum Axis {
    Axis_x,
    Axis_y,
    Axis_z,
    Axis_any(Vector3<f32>),
}

#[derive(Clone)]
pub struct Object {
    pub models: Vec<Model>,
    pub materials: Vec<Material>,
    pub textures: HashMap<String, TextureImages>,
    translate_matrix: Matrix4<f32>,
    rotate_matrix: Matrix4<f32>,
    scale_matrix: Matrix4<f32>,
    // model_transform_matrix: [[f32; 4]; 4],
    pub path: String,
    pub speed: Vector3<f32>,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Object {{
            \tmodels: {:?},
            \tmaterials: {:?},
            \ttextures: {:?},
            \ttranslate_matrix: {:?},
            \trotate_matrix: {:?},
            \tscale_matrix: {:?},
            \tpath: {:?},
            \tspeed: {:?},
             }}",
               self.models,
               self.materials,
               self.textures.iter().map(|(x,y)| {x.clone()}).collect::<String>(),//.iter().map|(x,y) {x}|.collect(),
               self.translate_matrix,
               self.rotate_matrix,
               self.scale_matrix,
               self.path,
               self.speed,
           )
    }
}


impl Object {
    pub fn new(models: Vec<Model>,
               materials: Vec<Material>,
               textures: HashMap<String, TextureImages>,
               path: String)
               -> Object {
        Object {
            models: models,
            materials: materials,
            textures: textures,
            translate_matrix: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
            rotate_matrix: Matrix4::from_angle_x(Rad(0.0)),
            scale_matrix: Matrix4::from_scale(0.6),
            path: path,
            speed: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    fn set_transform(&mut self) {
        let dx = self.speed[0].clone() * 2.0;
        let dy = self.speed[1].clone() * 2.0;
        let dz = self.speed[2].clone() * 2.0;
        self.translate(dx, dy, dz);
        self.rotate_matrix = Matrix4::from_angle_y(Deg(dx * 100.));

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

type PlayerID = usize;
type ModelID = usize;

pub struct LogicSystem {
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
    mouse_x: i32,
    mouse_y: i32,
    scene: Scene,
    player_to_model_mapping: HashMap<PlayerID, ModelID>,
}

impl System for LogicSystem {
    fn init(&mut self) {
        use MsgContent::*;
        use model::ModelMsg::*;
        let light = Light {
            position: [0.0, 1.0, 0.0, 1.0],
            color: [1.0, 1.0, 1.0],
            attenuation: [1.0, 0.5, 0.5],
            radius: 1.0,
        };
        self.scene.lights.push(light);
        let mut static_object_path: Vec<String> = Vec::new();

        static_object_path.push("assets/model/jet1.obj".to_string());
        static_object_path.push("assets/model/cube.obj".to_string());
        //static_object_path.push("assets/model/teapot.obj".to_string());

        for path in static_object_path {
            let msg = Msg { content: Logic(LogicMsg::ModelReq(path.clone())) };
            self.msg_tx[3].send(msg);
            //println!("TRY TO RECV MSG...");
            if let Ok(msg) = self.msg_rx.recv() {
                match msg.content {
                    Model(ObjectResult(Some(obj))) => {
                        self.scene.objects.push(obj);
                    }
                    Model(ObjectResult(None)) => unimplemented!(),
                    c => {
                        println!("{:?}", c);
                    }
                }
            }
            //println!("TRY TO RECV MSG... [FIN]");
        }
        let perspective = cgmath::Perspective {
            left: -4.0,
            right: 4.0,
            bottom: -4.0,
            top: 4.0,
            near: 1.0,
            far: 20.0,
        };
        let perspectivefov = cgmath::PerspectiveFov {
            fovy: Rad(3.14159 / 3.0),
            aspect: 1.0,
            near: 1.0,
            far: 20.0,
        };
        self.scene.camera = Camera::new(Point3::new(0.0, 0.0, 20.0),
                                        Point3::new(0.0, 0.0, 0.0),
                                        Vector3::new(0.0, 1.0, 0.0));
        self.scene
            .camera
            .set_projection_matrix(cgmath::Matrix4::from(perspectivefov));
        println!("Model Matrix: {:?}",
                 self.scene.objects[0].get_model_matrix());
        println!("View Matrix: {:?}", self.scene.camera.get_view_matrix());
        println!("Proj Matrix: {:?}",
                 self.scene.camera.get_projection_matrix());
        println!("Logic System Initilized.");
        self.msg_tx[1].send(Msg { content: MsgContent::System(SystemMsg::SysInit) });
        self.scene.objects[1].speed[1] = 0.01;
        /*
        let skybox = Skybox::new(String::from("assets/skybox/posx.jpg"),
                                 String::from("assets/skybox/posy.jpg"),
                                 String::from("assets/skybox/posz.jpg"),
                                 String::from("assets/skybox/negx.jpg"),
                                 String::from("assets/skybox/negy.jpg"),
                                 String::from("assets/skybox/negz.jpg"));
        self.scene.skybox = Some(skybox);
*/
    }
    fn main_loop(&mut self) {
        use MsgContent::*;
        let mut should_run = true;
        let mut render_ready = false;
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
                        println!("LOGIC Received SYSHALT");
                        should_run = false;
                    }
                    MsgContent::Render(RenderMsg::RenderResult(r)) => {
                        if !r.is_some() {
                            println!("Recv msg from Renderer: Fail to render...");
                        }
                        render_ready = true;
                    }
                    MsgContent::Input(InputMsg::KeyUp(key)) => {
                        use glium::glutin::VirtualKeyCode::*;
                        match key {
                            Left | Right => {
                                self.scene.objects[0].speed[0] = 0.0;
                            }
                            Up | Down => {
                                self.scene.objects[0].speed[1] = 0.0;
                            }
                            _ => {}
                        }
                    }
                    MsgContent::Input(InputMsg::KeyDown(key)) => {
                        use glium::glutin::VirtualKeyCode::*;
                        match key {
                            Left => {
                                //println!("{:?}", self.scene.objects[0].get_model_matrix());
                                //self.scene.objects[0].rotate(Axis::Axis_y, -5.0);
                                self.scene.objects[0].speed[0] = -0.1;
                            }
                            Right => {
                                //println!("{:?}", self.scene.objects[0].get_model_matrix());
                                //self.scene.objects[0].rotate(Axis::Axis_y, 5.0);
                                self.scene.objects[0].speed[0] = 0.1;
                            }
                            Up => {
                                self.scene.objects[0].speed[1] = 0.1;
                            }
                            Down => {
                                self.scene.objects[0].speed[1] = -0.1;
                            }
                            Space => {
                                println!("Shoot!");
                            }
                            RBracket => {
                                //println!("{:?}", self.scene.camera);
                                self.scene.camera.zoom(1.0);
                            }
                            LBracket => {
                                //println!("{:?}", self.scene.camera);
                                self.scene.camera.zoom(-1.0);
                            }
                            W => {
                                //println!("{:?}", self.scene.camera);
                                //self.scene.camera.move_y(1.0);
                            }
                            S => {
                                //println!("{:?}", self.scene.camera);
                                //self.scene.camera.move_y(-1.0);
                            }
                            Esc => {
                                should_run = false;
                                for tx in &self.msg_tx {
                                    tx.send(Msg {
                                                content: MsgContent::System(SystemMsg::SysHalt),
                                            });
                                }
                            }
                            _ => {}
                        }
                    }
                    c => {}
                }
            }
            for object in &mut self.scene.objects {
                object.set_transform();
            }
            // Pass to renderer
            if render_ready {
                let msg = Msg { content: Logic(LogicMsg::SceneSnd(self.scene.clone())) };
                self.msg_tx[1].send(msg);
                render_ready = false;
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
            scene: Scene::new(Vec::new(),
                              Vec::new(),
                              Camera::new(Point3::new(0.0, 0.0, 1.0),
                                          Point3::new(0.0, 0.0, 0.0),
                                          Vector3::new(0.0, 1.0, 0.0)),
                              None),
            player_to_model_mapping: HashMap::new(),
        }
    }

    fn register_user_model(&mut self, user_id: PlayerID, model_id: ModelID) {
        // TODO: register a model with a player
    }
}
