extern crate glfw;
use glfw::{Action, Context, Key};

pub struct Window {
    context: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>
}
 
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}


pub fn new(dimensions: (u32, u32), title: String, window_mode: glfw::WindowMode) -> Option<Window>
{
    // Create GLFW context
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to create GLFW context!");
        
    // Create window and event bindings
    let (mut window, events) = glfw.create_window(dimensions.0, dimensions.1, &title, window_mode).expect("Failed to create GLFW window!");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

    Some(Window{context: glfw, window: window, events: events})
}