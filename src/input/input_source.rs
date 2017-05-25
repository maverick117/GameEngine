
pub enum InputType {
    PollEvents,
    Interrupt,
}

pub struct InputSource {
    input_type: InputType,
}

impl InputSource {
    // Constructor, constructs a new input source for the system to handle
    pub fn new(input_type: InputType) -> InputSource {
        InputSource { input_type: input_type }
    }

    pub fn poll_events() {
        unimplemented!()
    }
}
