//! JoInn workspace sketch. The window only draws the model and sends it edits.

#![forbid(unsafe_code)]

mod edit;
mod layout;
mod view;

fn main() {
    view::launch();
}
