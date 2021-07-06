extern crate abyss_engine;
use abyss_engine::{Context};

extern crate glfw;
use glfw::{Context as GLFW_Context, *};

fn main() {
    let mut context = Context::new((500, 500), String::from("test"), glfw::WindowMode::Windowed);

    std::process::exit(
        match context.start_main_loop() {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("{:?}", err);
                1
            }
        }
    );
}
