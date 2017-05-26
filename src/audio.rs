
use baal;
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};

use super::System;
use super::Msg;
use super::*;

pub struct AudioSystem {
    msg_tx: Vec<Sender<Msg>>,
    msg_rx: Receiver<Msg>,
    setting: baal::Setting,
}

impl System for AudioSystem {
    fn init(&mut self) {
        baal::init(&self.setting).unwrap();
        baal::music::play(0);
        baal::effect::set_listener([1., 1., 1.]);

    }
    fn main_loop(&mut self) {
        let mut should_run = true;
        while should_run {
            let mut cmd_queue = Vec::new();
            while let Ok(msg) = self.msg_rx.try_recv() {
                cmd_queue.push(msg);
            }
            continue;
            for m in cmd_queue {
                self.msg_tx[1]
                    .send(Msg {
                              content: MsgContent::Debug(format!("Audio System received {:?}", m)),
                          })
                    .unwrap();
                match m.content {
                    MsgContent::System(SystemMsg::SysHalt) => {
                        should_run = false;
                    }
                    c => {}
                }
            }
        }
        baal::close();
    }
}

impl AudioSystem {
    pub fn new(msg_tx: Vec<Sender<Msg>>, msg_rx: Receiver<Msg>) -> AudioSystem {
        AudioSystem {
            msg_tx: msg_tx,
            msg_rx: msg_rx,
            setting: baal::Setting {
                effect_dir: "assets/fx".into(),
                music_dir: "assets/stream".into(),

                global_volume: 1.0,
                music_volume: 1.0,
                effect_volume: 1.0,

                distance_model: baal::effect::DistanceModel::Linear(10., 100.),

                music_transition: baal::music::MusicTransition::Instant,
                short_effects: vec!["wowa-intro.ogg".into()],
                persistent_effects: vec!["wowa-intro.ogg".into()],
                musics: vec!["to_be_free.ogg".into()],
            },
        }
    }
}
