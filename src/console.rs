use glutin;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ConsoleSystem {
    events_arc : Arc<Mutex<glutin::EventsLoop>>,
}

impl ConsoleSystem {
	pub fn new(events_arc: Arc<Mutex<glutin::EventsLoop>>) -> ConsoleSystem {
		ConsoleSystem {
			events_arc : events_arc,
		}
	}
}