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

        let light_output = &[("light_output", &light_texture)];

        let mut light_buffer =
            glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(&self.window,
                                                                          light_output
                                                                              .iter()
                                                                              .cloned(),
                                                                          &depthtexture)
                    .unwrap();

        // Load the skybox textures
        let skybox = Skybox::new(String::from("assets/skybox/posx.jpg"),
                                 String::from("assets/skybox/posy.jpg"),
                                 String::from("assets/skybox/posz.jpg"),
                                 String::from("assets/skybox/negx.jpg"),
                                 String::from("assets/skybox/negy.jpg"),
                                 String::from("assets/skybox/negz.jpg"));

        let dim = skybox.positive_x.dimensions();
        let skybox_px_tex = glium::texture::Texture2d::new(&self.window, glium::texture::RawImage2d::from_raw_rgba_reversed(skybox.positive_x.clone().into_raw(),dim)).unwrap();
        let dim = skybox.positive_y.dimensions();
        let skybox_py_tex = glium::texture::Texture2d::new(&self.window, glium::texture::RawImage2d::from_raw_rgba_reversed(skybox.positive_y.clone().into_raw(),dim)).unwrap();
        let dim = skybox.positive_z.dimensions();
        let skybox_pz_tex = glium::texture::Texture2d::new(&self.window, glium::texture::RawImage2d::from_raw_rgba_reversed(skybox.positive_z.clone().into_raw(),dim)).unwrap();
        let dim = skybox.negative_x.dimensions();
        let skybox_nx_tex = glium::texture::Texture2d::new(&self.window, glium::texture::RawImage2d::from_raw_rgba_reversed(skybox.negative_x.clone().into_raw(),dim)).unwrap();
        let dim = skybox.negative_y.dimensions();
        let skybox_ny_tex = glium::texture::Texture2d::new(&self.window, glium::texture::RawImage2d::from_raw_rgba_reversed(skybox.negative_y.clone().into_raw(),dim)).unwrap();
        let dim = skybox.negative_z.dimensions();
        let skybox_nz_tex = glium::texture::Texture2d::new(&self.window, glium::texture::RawImage2d::from_raw_rgba_reversed(skybox.negative_z.clone().into_raw(),dim)).unwrap();

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
                                                 &mut light_buffer,
                                                 &texture1,
                                                 &texture2,
                                                 &texture3,
                                                 &texture4,
                                                 &light_texture,
                                                 &skybox_px_tex,
                                                 &skybox_py_tex,
                                                 &skybox_pz_tex,
                                                 &skybox_nx_tex,
                                                 &skybox_ny_tex,
                                                 &skybox_nz_tex);
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
                  light_buffer: &mut glium::framebuffer::MultiOutputFrameBuffer,
                  texture1: &glium::texture::Texture2d,
                  texture2: &glium::texture::Texture2d,
                  texture3: &glium::texture::Texture2d,
                  texture4: &glium::texture::Texture2d,
                  light_texture: &glium::texture::Texture2d,
                  skybox_px_tex: &glium::texture::Texture2d,
                  skybox_py_tex: &glium::texture::Texture2d,
                  skybox_pz_tex: &glium::texture::Texture2d,
                  skybox_nx_tex: &glium::texture::Texture2d,
                  skybox_ny_tex: &glium::texture::Texture2d,
                  skybox_nz_tex: &glium::texture::Texture2d)
                  -> Option<()> {

        #[derive(Copy, Clone, Debug)]
        struct Vertex {
            position: [f32; 3],
            normal: [f32; 3],
            color_diffuse: [f32; 3],
            color_specular: [f32; 3],
            shininess: f32,
            texcoord: [f32; 2],
        }

        implement_vertex!(Vertex,
                          position,
                          normal,
                          color_diffuse,
                          color_specular,
                          shininess,
                          texcoord);

        let mut target = self.window.draw();
        // Equivalent to glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // Single pass on all objects
        let mut vertex_data: Vec<Vertex> = Vec::new();
        let mut index_data: Vec<u32> = Vec::new();
        framebuffer.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);



        for object in scene.objects {
            for model in &object.models {
                vertex_data.clear(); // Clear out vertex data from previous pass
                index_data.clear(); // Clear out index data from the previous pass
                let mesh = &model.mesh;

                let color_diffuse: [f32; 3];
                let color_specular: [f32; 3];
                let shininess: f32;
                let diffuse_tex: glium::texture::Texture2d;

                if let Some(x) = mesh.material_id {
                    let mt = &object.materials[x];
                    // Retrieve the material
                    color_diffuse = mt.diffuse.clone();
                    color_specular = mt.specular.clone();
                    shininess = mt.shininess;
                    let diffuse_image = &object
                                             .textures
                                             .get(&mt.diffuse_texture)
                                             .unwrap()
                                             .diffuse_image;
                    let image_dimensions = diffuse_image.dimensions();
                    diffuse_tex =
                        glium::texture::Texture2d::new(&self.window, glium::texture::RawImage2d::from_raw_rgba_reversed(diffuse_image
                                                                               .clone()
                                                                               .into_raw(),
                                                                           image_dimensions)).unwrap();
                } else {
                    color_diffuse = [0.6, 0.6, 0.6];
                    color_specular = [0.3, 0.3, 0.3];
                    shininess = 25.0;
                    diffuse_tex = glium::texture::Texture2d::empty(&self.window, 2, 2).unwrap();
                    diffuse_tex.write(glium::Rect {
                                          left: 0,
                                          bottom: 0,
                                          width: 2,
                                          height: 2,
                                      },
                                      vec![vec![0., 0.], vec![0., 0.]]);
                }

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

                    vertex_data.push(Vertex {
                                         position: pos,
                                         normal: normal,
                                         color_diffuse: color_diffuse,
                                         color_specular: color_specular,
                                         shininess: shininess,
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
                    diffuse_tex: &diffuse_tex,
                    //position: [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z],
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

                // Draw to gBuffer
                framebuffer
                    .draw(&pre_pass_vertex_buffer,
                          //&pre_pass_index_buffer,
                          &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                          pre_pass_program,
                          &uniforms,
                          &params)
                    .unwrap();
            }
        }

        // Quad buffer for further passes
        let quad_vertex_buffer = {
            #[derive(Copy,Clone,Debug)]
            struct Vertex {
                position: [f32; 4],
                texcoord: [f32; 2],
            }

            implement_vertex!(Vertex, position, texcoord);

            glium::VertexBuffer::new(&self.window,
                                     &[Vertex {
                                          position: [-1.0, -1.0, 0.0, 1.0],
                                          texcoord: [0.0, 0.0],
                                      },
                                      Vertex {
                                          position: [1.0, -1.0, 0.0, 1.0],
                                          texcoord: [1.0, 0.0],
                                      },
                                      Vertex {
                                          position: [1.0, 1.0, 0.0, 1.0],
                                          texcoord: [1.0, 1.0],
                                      },
                                      Vertex {
                                          position: [-1.0, 1.0, 0.0, 1.0],
                                          texcoord: [0.0, 1.0],
                                      }])
                    .unwrap()
        };
        use glium::index::PrimitiveType;
        let quad_index_buffer = glium::IndexBuffer::new(&self.window,
                                                        PrimitiveType::TrianglesList,
                                                        &[0u16, 1, 2, 0, 2, 3])
                .unwrap();


        // Shading Passes

        // Shading parameters
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
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
            ..Default::default()
        };

        light_buffer.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        // Get the eye position
        let eye_position = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];

        for light in scene.lights {
            let light_uniforms = uniform!{
                eyePos: eye_position,
                lightPos: light.position,
                lightColor: light.color,
                attenuation: light.attenuation,
                radius: light.radius,
                gPosition: texture1,
                gNormal: texture2,
            };
            light_buffer
                .draw(&quad_vertex_buffer,
                      &quad_index_buffer,
                      lighting_program,
                      &light_uniforms,
                      &Default::default())
                .unwrap();
        }

        // Composition Pass

        let comp_uniforms = uniform!{
            eyePos: eye_position,
            position_texture: texture1,
            normal_texture: texture2,
            albedo_texture: texture3,
            specular_texture: texture4,
            lighting_texture: light_texture,
            skybox_px_tex: skybox_px_tex,
            skybox_py_tex: skybox_py_tex,
            skybox_pz_tex: skybox_pz_tex,
            skybox_nx_tex: skybox_nx_tex,
            skybox_ny_tex: skybox_ny_tex,
            skybox_nz_tex: skybox_nz_tex,
        };

        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target
            .draw(&quad_vertex_buffer,
                  &quad_index_buffer,
                  composition_program,
                  &comp_uniforms,
                  &Default::default())
            .unwrap();

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

    pub fn get_eye_pos(&self) -> cgmath::Point3<f32> {
        self.eye
    }
}

use image;

#[derive(Debug, Clone)]
pub struct Skybox {
    positive_x: image::RgbaImage,
    positive_y: image::RgbaImage,
    positive_z: image::RgbaImage,
    negative_x: image::RgbaImage,
    negative_y: image::RgbaImage,
    negative_z: image::RgbaImage,
}

impl Skybox {
    pub fn new(px: String, py: String, pz: String, nx: String, ny: String, nz: String) -> Skybox {
        use std::io::Cursor;
        Skybox {
            positive_x: image::open(px.clone()).unwrap().to_rgba(),
            positive_y: image::open(py.clone()).unwrap().to_rgba(),
            positive_z: image::open(pz.clone()).unwrap().to_rgba(),
            negative_x: image::open(nx.clone()).unwrap().to_rgba(),
            negative_y: image::open(ny.clone()).unwrap().to_rgba(),
            negative_z: image::open(nz.clone()).unwrap().to_rgba(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub camera: Camera,
    pub skybox: Option<Skybox>,
}

impl Scene {
    pub fn new(objects: Vec<Object>,
               lights: Vec<Light>,
               camera: Camera,
               skybox: Option<Skybox>)
               -> Scene {
        Scene {
            objects: objects,
            lights: lights,
            camera: camera,
            skybox: skybox,
        }
    }
}
