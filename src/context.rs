extern crate glfw;
use glfw::{Context as GLFW_Context, *};

use crate::error::EngineError;

pub struct Context{
    glfw_context: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) -> Result<(), EngineError> {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
            Err(EngineError::RuntimeError(String::from("OH NO, an error!")))
        }
        _ => Ok(())
    }
}

impl Context {
    pub fn new(window_dimensions: (u32, u32), window_title: String, window_mode: glfw::WindowMode) -> Context
    {
        // Create GLFW context
        let glfw_context = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to create GLFW context!");
        
        // Create window and event bindings
        let (mut window, events) = glfw_context.create_window(window_dimensions.0, window_dimensions.1, &window_title, window_mode).expect("Failed to create GLFW window!");

        window.set_key_polling(true);
        window.make_current();

        Context{glfw_context, window, events}
    }

    pub fn start_main_loop(&mut self) -> Result<(), EngineError>
    {
        while !self.window.should_close() {
            // Handle input
            self.glfw_context.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                handle_window_event(&mut self.window, event)?;
            }
            // Update state
            // Render graphics
        }

        Ok(())
    }
}