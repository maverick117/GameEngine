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
    RenderResult(Option<()>),
}

pub struct RenderSystem {
    window: GlutinFacade,
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
}

impl System for RenderSystem {
    fn init(&mut self) {
        let mut target = self.window.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.2, 0.0), 1.0);
        target.finish();
    }
    fn main_loop(&mut self) {
        let mut should_run = true;
        while should_run {
            let event_list: Vec<Event> = self.window.poll_events().collect();
            for event in event_list {
                use glium::glutin::Event::*;
                use super::*;
                match event {
                    Resized(width, height) => {
                        println!("DEBUG: Window Resized: {:?} x {:?}", width, height);
                    }
                    Closed => {
                        use MsgContent;
                        use SystemMsg;
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
                    _ => {}
                }
            }

            let mut msg_list: Vec<Msg> = Vec::new();
            //msg_list.push(self.msg_rx.recv().unwrap());
            while let Ok(msg) = self.msg_rx.try_recv() {
                msg_list.push(msg);
            }
            for msg in msg_list {
                //println!("Render received: {:?}", msg);
                use Event::*;
                use MsgContent::*;
                use SystemMsg::*;
                let render_msg: Msg;
                use logic::LogicMsg::*;
                match msg.content {
                    System(SysHalt) => should_run = false,
                    Logic(SceneSnd(scene)) => {
                        let result = self.render(scene);
                        let render_msg = Msg { content: Render(RenderMsg::RenderResult(result)) };
                        self.msg_tx[2].send(render_msg);
                    }
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
                .with_dimensions(1000, 1000)
                .with_vsync()
                .with_depth_buffer(24)
                .build_glium()
                .unwrap(),
            msg_tx: msg_tx,
            msg_rx: msg_rx,
        }
    }
    pub fn render(&mut self, scene: Scene) -> Option<()> {
        let program = program!(&self.window,
            330 => {
                vertex: "
                    #version 330
                    uniform mat4 proj_matrix;
                    uniform mat4 view_matrix;
                    uniform mat4 model_matrix;

                    in vec3 position;
                    in vec3 normal;
                    in vec3 color_diffuse;
                    in vec4 color_specular;

                    out vec3 v_position;
                    out vec3 v_normal;
                    out vec3 v_color_diffuse;
                    out vec4 v_color_specular;

                    void main() {{
                        v_position = position;
                        v_normal = normal;
                        v_color_diffuse = color_diffuse;
                        v_color_specular = color_specular;
                        gl_Position = proj_matrix * view_matrix * model_matrix * vec4(v_position, 1.0);
                    }}
                ",
                fragment: "
                    #version 330

                    in vec3 v_position;
                    in vec3 v_normal;
                    in vec3 v_color_diffuse;
                    in vec4 v_color_specular;
                    // to implement
                    void main() {

                        vec3 lightPos = vec3(0.0, 1.0, 0.0);
                        vec3 lightColor = vec3(1.0, 1.0, 1.0);
                        vec3 objectColor = vec3(0.4, 0.3, 0.05);

                        // ambient color
                        vec3 ambient = 0.1 * lightColor;

                        // diffuse color
                        vec3 norm = normalize(v_normal);
                        vec3 lightDir = normalize(lightPos - v_position);
                        float diff = max(dot(norm, lightColor), 0.0);
                        vec3 diffuse = diff * lightColor;

                        vec3 result = (ambient + diffuse) * objectColor;
                        gl_FragColor = vec4(result, 1.0f);

                    }
                ",
            },
        ).unwrap();

        #[derive(Copy, Clone, Debug)]
        struct Vertex {
            position: [f32; 3],
            normal: [f32; 3],
            color_diffuse: [f32; 3],
            color_specular: [f32; 4],
        }

        implement_vertex!(Vertex, position, normal, color_diffuse, color_specular);
        let mut target = self.window.draw();
        let mut vertex_data: Vec<Vertex> = Vec::new();
        for object in scene.objects {

            for model in &object.models {
                let mesh = &model.mesh;
                //let mut vertex_data = Vec::new();

                for idx in &mesh.indices {
                    let i = *idx as usize;
                    let pos = [mesh.positions[3 * i],
                               mesh.positions[3 * i + 1],
                               mesh.positions[3 * i + 2]];
                    let normal = if !mesh.normals.is_empty() {
                        [mesh.normals[3 * i],
                         mesh.normals[3 * i + 1],
                         mesh.normals[3 * i + 2]]
                    } else {
                        [0.0, 0.0, 0.0]
                    };

                    let (color_diffuse, color_specular) = match mesh.material_id {
                        Some(i) => {
                            (object.materials[i].diffuse,
                             [object.materials[i].specular[0],
                              object.materials[i].specular[1],
                              object.materials[i].specular[2],
                              object.materials[i].shininess])
                        }
                        None => ([0.8, 0.8, 0.8], [0.15, 0.15, 0.15, 15.0]),
                    };
                    vertex_data.push(Vertex {
                                         position: pos,
                                         normal: normal,
                                         color_diffuse: color_diffuse,
                                         color_specular: color_specular,
                                     });
                }
            }
            let vertex_buffer = glium::vertex::VertexBuffer::new(&self.window, &vertex_data)
                .unwrap()
                .into_vertex_buffer_any();
            //println!("{:?}", scene.camera.get_projection_matrix());
            //println!("{:?}", scene.camera.get_view_matrix());
            //println!("{:?}", object.get_model_matrix());

            let uniforms = uniform! {
                proj_matrix: scene.camera.get_projection_matrix(),
                view_matrix: scene.camera.get_view_matrix(),
                model_matrix: object.get_model_matrix(),
            };
            // draw parameters
            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };


            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
            target
                .draw(&vertex_buffer,
                      &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                      &program,
                      &uniforms,
                      &params)
                .unwrap();

        }
        target.finish().unwrap();
        Some(()) // TODO: None
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
        use cgmath::Matrix;

        cgmath::Matrix4::look_at(self.eye, self.center, self.up).getArray()
    }
    pub fn get_projection_matrix(&self) -> [[f32; 4]; 4] {
        self.projection_matrix
    }

    pub fn set_projection_matrix(&mut self, m: cgmath::Matrix4<f32>) {
        use cgmath::Matrix;
        self.projection_matrix = m.transpose().getArray();
    }

    pub fn zoom(&mut self, dist: f32) {
        self.eye.z += dist;
    }

    pub fn move_y(&mut self, dist: f32) {
        self.eye.y += dist;
    }
}


#[derive(Clone, Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub camera: Camera,
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
