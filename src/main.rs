mod console;
mod render;
mod input;
mod model;

pub struct Msg {

}

pub trait System{
    fn handle_message(msg: Msg);


}

fn main() {
    println!("Hello, world!");
}
