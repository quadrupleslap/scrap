use super::Server;
use super::ffi::*;

#[derive(Debug)]
pub struct Display<'a> {
    server: &'a Server,
    default: bool,
    width: u16,
    height: u16,
    root: xcb_window_t
}

impl<'a> Display<'a> {
    pub unsafe fn new(
        server: &'a Server,
        default: bool,
        width: u16,
        height: u16,
        root: xcb_window_t
    ) -> Display<'a> {
        Display { server, default, width, height, root }
    }

    pub fn server(&self) -> &'a Server { self.server }
    pub fn is_default(&self) -> bool { self.default }
    pub fn width(&self) -> u16 { self.width }
    pub fn height(&self) -> u16 { self.height }
    pub fn root(&self) -> xcb_window_t { self.root }
}
