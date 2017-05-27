
use tobj;
use std::path::{Path, PathBuf};
use std::sync::mpsc::*;
use glium::vertex::VertexBufferAny;
use System;
use Msg;

#[derive(Clone,Debug)]
pub enum ModelMsg {
    ModelAck(bool),
    ModelResult(Option<tobj::Model>),
    MaterialAck(bool),
    MaterialResult(Option<tobj::Material>),
}

pub struct ModelSystem {
    models: Vec<tobj::Model>,
    materials: Vec<tobj::Material>,
    //model_path: PathBuf,
}

impl System for ModelSystem {
    fn init(&mut self) {
        let model_path = Path::new("assets/model");
        for entry in model_path.read_dir().expect("Read model directory failed.") {
            if let Ok(entry) = entry {
                let load_result = tobj::load_obj(&model_path);
                let (models, materials) = load_result.expect("Load object failed");
                for m in models {
                    self.models.push(m);
                }
                for m in materials {
                    self.materials.push(m);
                }

            }
        }
        println!("Model load finished.");
    }
    fn main_loop(&mut self) {
        let mut should_run = true;
        while should_run {}
    }
}

impl ModelSystem {
    pub fn new(msg_tx: Vec<Sender<Msg>>, msg_rx: Receiver<Msg>) -> ModelSystem {
        ModelSystem {
            models: Vec::new(),
            materials: Vec::new(),
            //model_path: PathBuf::new(),
        }
    }
}
