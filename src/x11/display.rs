use std::rc::Rc;
use super::Server;
use super::ffi::*;

#[derive(Debug)]
pub struct Display {
    server: Rc<Server>,
    default: bool,
    width: u16,
    height: u16,
    root: xcb_window_t
}

impl Display {
    pub unsafe fn new(
        server: Rc<Server>,
        default: bool,
        width: u16,
        height: u16,
        root: xcb_window_t
    ) -> Display {
        Display { server, default, width, height, root }
    }

    pub fn server(&self) -> &Rc<Server> { &self.server }
    pub fn is_default(&self) -> bool { self.default }
    pub fn width(&self) -> u16 { self.width }
    pub fn height(&self) -> u16 { self.height }
    pub fn root(&self) -> xcb_window_t { self.root }
}
