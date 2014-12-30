#![feature(unboxed_closures)]
#![feature(unsafe_destructor)]
#![feature(slicing_syntax)]

extern crate rustbox;
extern crate gapbuffer;

pub use editor::Editor;
pub use input::Input;
pub use frontends::RustboxFrontend;

mod input;
mod utils;
mod buffer;
mod editor;
mod keyboard;
mod keymap;
mod view;
mod uibuf;
mod log;
mod frontends;
mod textobject;

#[deriving(Copy)]
pub enum Response {
    Continue,
    Quit,
}
