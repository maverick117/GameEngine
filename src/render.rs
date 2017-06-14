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
// use glium_text;
use std;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

static window_width: u32 = 1000;
static window_height: u32 = 1000;

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
        use MsgContent;
        use SystemMsg;
        loop {
            if let Msg { content: MsgContent::System(SystemMsg::SysInit) } =
                self.msg_rx.recv().unwrap() {
                break;
            }
        }
        use MsgContent::Render;
        let render_msg = Msg { content: Render(RenderMsg::RenderResult(Some(()))) };
        self.msg_tx[2].send(render_msg);



    }
    fn main_loop(&mut self) {
        let mut should_run = true;

        // Read in vertex and fragment shaders from file
        let mut pre_pass_vs = String::new();
        let mut pre_pass_fs = String::new();
        let mut lighting_vs = String::new();
        let mut lighting_fs = String::new();
        let mut composition_vs = String::new();
        let mut composition_fs = String::new();

        {
            let file = File::open("shaders/pre_pass_program.vs").unwrap();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut pre_pass_vs);
        }

        {
            let file = File::open("shaders/pre_pass_program.fs").unwrap();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut pre_pass_fs);
        }
        {
            let file = File::open("shaders/lighting.vs").unwrap();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut lighting_vs);
        }

        {
            let file = File::open("shaders/lighting.fs").unwrap();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut lighting_fs);
        }
        {
            let file = File::open("shaders/composition.vs").unwrap();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut composition_vs);
        }

        {
            let file = File::open("shaders/composition.fs").unwrap();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut composition_fs);
        }

        // Generate the shader programs
        let pre_pass_program =
            glium::Program::from_source(&self.window, &pre_pass_vs, &pre_pass_fs, None).unwrap();
        let lighting_program =
            glium::Program::from_source(&self.window, &lighting_vs, &lighting_fs, None).unwrap();
        let composition_program =
            glium::Program::from_source(&self.window, &composition_vs, &composition_fs, None)
                .unwrap();

        // Generate renderable textures for the scene

        // Position
        let texture1 = glium::texture::Texture2d::empty_with_format(
            &self.window,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap,
            window_width,
            window_height)
            .unwrap();

        // Normals
        let texture2 = glium::texture::Texture2d::empty_with_format(
            &self.window,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap,
            window_width,
            window_height)
            .unwrap();

        // Albedo
        let texture3 = glium::texture::Texture2d::empty_with_format(
            &self.window,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap,
            window_width,
            window_height)
            .unwrap();

        // Specular
        let texture4 = glium::texture::Texture2d::empty_with_format(
            &self.window,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap,
            window_width,
            window_height)
            .unwrap();

        // Depth buffer
        let depthtexture = glium::texture::DepthTexture2d::empty_with_format(
            &self.window,
            glium::texture::DepthFormat::F32,
            glium::texture::MipmapsOption::NoMipmap,
            window_width,
            window_height)
            .unwrap();

        let output = &[("gPosition", &texture1),
                       ("gNormal", &texture2),
                       ("gAlbedo", &texture3),
                       ("gSpec", &texture4)];

        // The gBuffer
        let mut framebuffer =
            glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(&self.window,
                                                                          output.iter().cloned(),
                                                                          &depthtexture)
                    .unwrap();

        let light_texture = glium::texture::Texture2d::empty_with_format(&self.window,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap, window_width, window_height).unwrap();

        let mut light_buffer =
            glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&self.window,
                                                                     &light_texture,
                                                                     &depthtexture)
                    .unwrap();


        // Main loop
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
                        let result = self.render(scene,
                                                 &pre_pass_program,
                                                 &lighting_program,
                                                 &composition_program,
                                                 &mut framebuffer,
                                                 &mut light_buffer);
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
                .with_dimensions(window_width, window_height)
                .with_vsync()
                .with_depth_buffer(24)
                .build_glium()
                .unwrap(),
            msg_tx: msg_tx,
            msg_rx: msg_rx,
        }
    }

    pub fn render(&mut self,
                  scene: Scene,
                  pre_pass_program: &glium::Program,
                  lighting_program: &glium::Program,
                  composition_program: &glium::Program,
                  framebuffer: &mut glium::framebuffer::MultiOutputFrameBuffer,
                  light_buffer: &mut glium::framebuffer::SimpleFrameBuffer)
                  -> Option<()> {

        #[derive(Copy, Clone, Debug)]
        struct Vertex {
            position: [f32; 3],
            normal: [f32; 3],
            color_diffuse: [f32; 3],
            color_specular: [f32; 4],
            texcoord: [f32; 2],
        }

        implement_vertex!(Vertex,
                          position,
                          normal,
                          color_diffuse,
                          color_specular,
                          texcoord);

        let mut target = self.window.draw();
        // Equivalent to glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // TODO: Retrieve texture info from each model

        // Single pass on all objects
        let mut vertex_data: Vec<Vertex> = Vec::new();
        let mut index_data: Vec<u32> = Vec::new();
        framebuffer.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        for object in scene.objects {
            for model in &object.models {
                vertex_data.clear(); // Clear out vertex data from previous pass
                index_data.clear(); // Clear out index data from the previous pass
                let mesh = &model.mesh;
                for idx in &mesh.indices {
                    // For triangle indices
                    let i = *idx as usize;
                    index_data.push(i as u32);
                    // Get the vertice positions
                    let pos = [mesh.positions[3 * i],
                               mesh.positions[3 * i + 1],
                               mesh.positions[3 * i + 2]];
                    // And the normals
                    let normal = if !mesh.normals.is_empty() {
                        [mesh.normals[3 * i],
                         mesh.normals[3 * i + 1],
                         mesh.normals[3 * i + 2]]
                    } else {
                        [0.0, 0.0, 0.0]
                    };
                    // And the texcoords
                    let texcoord = if !mesh.texcoords.is_empty() {
                        [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]]
                    } else {
                        [0.0, 0.0]
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
                                         texcoord: texcoord,
                                     });
                }
                // Draw for each model
                let pre_pass_vertex_buffer = glium::VertexBuffer::new(&self.window, &vertex_data)
                    .unwrap();

                use glium::index::PrimitiveType;
                let pre_pass_index_buffer = glium::IndexBuffer::new(&self.window,
                                                                    PrimitiveType::TrianglesList,
                                                                    &index_data)
                        .unwrap();

                // TODO: Modify uniforms to comply with shader profiles

                let uniforms = uniform! {
                    proj_matrix: scene.camera.get_projection_matrix(),
                    view_matrix: scene.camera.get_view_matrix(),
                    model_matrix: object.get_model_matrix(),
                    // TODO: Add texture in here
                    //position: [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z],
                };

                // Draw to gBuffer
                framebuffer
                    .draw(&pre_pass_vertex_buffer,
                          &pre_pass_index_buffer,
                          pre_pass_program,
                          &uniforms,
                          &Default::default())
                    .unwrap();
            }
        }


        // Shading Passes
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            },
            ..Default::default()
        };

        light_buffer.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        // Get the eye position
        let eye_position = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];

        for light in scene.lights {
            let light_uniforms = uniform!{
                // TODO: uniforms to pass to the lighting pass
            };
        }



        /*
        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program,
                    &uniforms,
                    &params)
                    .unwrap();
        */
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
