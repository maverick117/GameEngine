use glium;
use glium::glutin;

use std::sync::mpsc::*;
use super::System;
use super::Msg;
use std::sync::Arc;
use std::sync::Mutex;
use glium::backend::glutin_backend::GlutinFacade;
use glium::DisplayBuild;
use glium::glutin::Event;
use cgmath;
use logic::*;
use tool::*;
use glium::Surface;

#[derive(Clone,Debug)]
pub enum RenderMsg {
    InputQueue(Vec<Event>),
}

pub struct RenderSystem {
    window: GlutinFacade,
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
}

impl System for RenderSystem {
    fn init(&mut self) {}
    fn main_loop(&mut self) {
        let mut should_run = true;
        while should_run {
            let event_list: Vec<Event> = self.window.poll_events().collect();
            if event_list.len() > 0 {
                println!("{:?}", event_list);
                use MsgContent::*;
                let event_msg = Msg { content: Render(RenderMsg::InputQueue(event_list)) };
                self.msg_tx[0].send(event_msg).unwrap();
            }
            let mut msg_list: Vec<Msg> = Vec::new();
            while let Ok(msg) = self.msg_rx.try_recv() {
                msg_list.push(msg);
            }
            for msg in msg_list {
                println!("Render received: {:?}", msg);
                use Event::*;
                use MsgContent::*;
                use SystemMsg::*;
                let render_msg: Msg;
                use logic::LogicMsg::*;
                match msg.content {
                    System(SysHalt) => should_run = false,
                    Logic(SceneSnd(scene)) => render_msg = self.render(scene),
                    _ => {}
                }


            }
        }
        println!("Render exited");
    }
}

impl RenderSystem {
    pub fn new(msg_tx: Vec<Sender<Msg>>, msg_rx: Receiver<Msg>) -> RenderSystem {
        RenderSystem {
            window: glutin::WindowBuilder::new()
                .with_title("Engine Demo".to_string())
                .with_dimensions(1024, 768)
                .with_vsync()
                .with_depth_buffer(24)
                .build_glium()
                .unwrap(),
            msg_tx: msg_tx,
            msg_rx: msg_rx,
        }
    }
    pub fn render(&mut self, scene: Scene) -> Msg {
        let mut target = self.window.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        for object in scene.objects {
            for model in object.models {
                let mesh = &model.mesh;
                #[derive(Copy, Clone)]
                struct Vertex {
                    position: [f32; 3],
                    normal: [f32; 3],
                    texture: [f32; 2],
                }

                implement_vertex!(Vertex, position, normal, texture);

                // let vertex_data : Vec<Vertex> = mesh.indices.iter().map(|i| {
                //     let i = i as usize;
                //     let normal:[f32; 3] = [1., 1., 1.];
                //     let texture:[f32; 2] = [0., 0.];
                //     let position = [mesh.positions[i * 3], mesh.positions[i * 3 + 1], mesh.positions[i * 3 + 2]];
                //     if !mesh.normals.is_empty() {
                //         // normal = [x, y, z]
                //         normal = [mesh.normals[i * 3], mesh.normals[i * 3 + 1],
                //                       mesh.normals[i * 3 + 2]];
                //     }

                //     if !mesh.texcoords.is_empty() {
                //         // texcoord = [u, v];
                //         texture = [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]];
                //     }

                //     Vertex {
                //         position: position,
                //         normal: normal,
                //         texture: texture,
                //     }
                // }).collect::<Vertex>().to_vec();

                let mut vertex_data = Vec::new();
                for i in &mesh.indices {
                    let i = *i as usize;
                    let mut normal: [f32; 3] = [1., 1., 1.];
                    let mut texture: [f32; 2] = [0., 0.];
                    let position = [mesh.positions[i * 3],
                                    mesh.positions[i * 3 + 1],
                                    mesh.positions[i * 3 + 2]];
                    if !mesh.normals.is_empty() {
                        // normal = [x, y, z]
                        normal = [mesh.normals[i * 3],
                                  mesh.normals[i * 3 + 1],
                                  mesh.normals[i * 3 + 2]];
                    }

                    if !mesh.texcoords.is_empty() {
                        // texcoord = [u, v];
                        texture = [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]];
                    }

                    vertex_data.push(Vertex {
                                         position: position,
                                         normal: normal,
                                         texture: texture,
                                     });
                }

                let vertex_buffer = glium::vertex::VertexBuffer::new(&self.window, &vertex_data)
                    .unwrap()
                    .into_vertex_buffer_any();
                // target.draw(&vertex_buffer,
                //     &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                //     &program, &uniforms, &params).unwrap();
                unimplemented!()
            }
        }
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct Camera {
    eye: cgmath::Point3<f32>,
    center: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    projection_matrix: [[f32; 4]; 4],
}

impl Camera {
    pub fn new(eye: cgmath::Point3<f32>,
               center: cgmath::Point3<f32>,
               up: cgmath::Vector3<f32>)
               -> Camera {

        let m = cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 0.0, 0.0));
        Camera {
            eye: eye,
            center: center,
            up: up,
            projection_matrix: m.getArray(),
        }
    }
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        cgmath::Matrix4::look_at(self.eye, self.center, self.up).getArray()
    }
    pub fn get_projection_matrix(&self) -> [[f32; 4]; 4] {
        self.projection_matrix
    }
}


#[derive(Clone, Debug)]
pub struct Scene {
    objects: Vec<Object>,
    lights: Vec<Light>,
    camera: Camera,
}

impl Scene {
    pub fn new(objects: Vec<Object>, lights: Vec<Light>, camera: Camera) -> Scene {
        Scene {
            objects: objects,
            lights: lights,
            camera: camera,
        }
    }
}
