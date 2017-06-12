
use tobj;
use std::path::{Path, PathBuf};
use std::sync::mpsc::*;
use glium::vertex::VertexBufferAny;
use glium::glutin::Event;
use logic::Object;
use System;
use Msg;

#[derive(Clone,Debug)]
pub enum ModelMsg {
    ObjectResult(Option<Object>),
}

pub struct ModelSystem {
    objects: Vec<Object>,
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
    //model_path: PathBuf,
}

impl System for ModelSystem {
    fn init(&mut self) {
        println!("Loading models");
        let model_path = Path::new("assets/model");
        for entry in model_path.read_dir().expect("Read model directory failed.") {
            if let Ok(entry) = entry {
                //println!("{:?}", entry);
                let load_result = tobj::load_obj(&entry.path());
                let (models, materials) = load_result.expect("Load object failed");
                self.objects
                    .push(Object::new(models,
                                      materials,
                                      entry.path().into_os_string().into_string().unwrap()));
            }
        }
        // println!("Model load finished. Objects: {:?}", self.objects);
    }
    fn main_loop(&mut self) {
        let mut should_run = true;
        while should_run {
            let mut msg_list: Vec<Msg> = Vec::new();
            msg_list.push(self.msg_rx.recv().unwrap());
            while let Ok(msg) = self.msg_rx.try_recv() {
                msg_list.push(msg);
            }
            for msg in msg_list {
                // println!("Model received: {:?}", msg);
                use Event::*;
                use MsgContent::*;
                use SystemMsg::*;
                use logic::LogicMsg::*;
                match msg.content {
                    System(SysHalt) => should_run = false,
                    Logic(ModelReq(s)) => {
                        let mut return_value = Option::None;
                        for object in &self.objects {
                            if object.path == s {
                                return_value = Some(object.clone());
                                break;
                            }
                        }
                        let model_msg =
                            Msg { content: Model(ModelMsg::ObjectResult(return_value)) };
                        self.msg_tx[3].send(model_msg);
                    }
                    _ => {
                        unimplemented!();
                    }
                }
            }
        }
        println!("Model exited.");
    }
}

impl ModelSystem {
    pub fn new(msg_tx: Vec<Sender<Msg>>, msg_rx: Receiver<Msg>) -> ModelSystem {
        ModelSystem {
            objects: Vec::new(),
            msg_tx: msg_tx,
            msg_rx: msg_rx,
            //model_path: PathBuf::new(),
        }
    }
}
