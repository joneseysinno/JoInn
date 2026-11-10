//! JoInn workspace sketch. The window only draws the model and sends it edits.

#![forbid(unsafe_code)]

mod camera;
mod edit;
mod layout;
mod view;
mod wires;

fn main() {
    view::launch();
}
