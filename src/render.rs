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
                match msg.content {
                    System(SysHalt) => should_run = false,
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
}

#[derive(Clone, Debug)]
pub struct Camera {
    eye: cgmath::Point3<f32>,
    center: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
}

impl Camera {
    pub fn new(eye: cgmath::Point3<f32>, center: cgmath::Point3<f32>, up: cgmath::Vector3<f32>) -> Camera {
        Camera {
            eye: eye,
            center: center,
            up: up,
        }
    }
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        cgmath::Matrix4::look_at(self.eye, self.center, self.up).getArray()
    }
    pub fn get_perspective_matrix(&self, perspective: cgmath::Perspective<f32>) -> [[f32; 4]; 4] {
        cgmath::Matrix4::from(perspective).getArray()
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
        Scene { objects: objects,
                lights: lights,
                camera: camera,
            }
    }
}