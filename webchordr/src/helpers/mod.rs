mod class;

pub use class::Class;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("Could not detect the JS window object")
}
