mod class;
mod tri;

pub use class::Class;
pub use tri::Tri;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("Could not detect the JS window object")
}
