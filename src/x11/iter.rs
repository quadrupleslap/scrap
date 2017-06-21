use std::rc::Rc;
use super::{Display, Server};
use super::ffi::*;

pub struct DisplayIter {
    raw: xcb_screen_iterator_t,
    index: i32,
    default_index: i32,
    server: Rc<Server>
}

impl DisplayIter {
    pub unsafe fn new(
        raw: xcb_screen_iterator_t,
        default_index: i32,
        server: Rc<Server>
    ) -> DisplayIter {
        DisplayIter { raw, index: 0, default_index, server }
    }
}

impl Iterator for DisplayIter {
    type Item = Display;

    fn next(&mut self) -> Option<Display> {
        if self.raw.rem == 0 {
            return None;
        }

        unsafe {
            let data = &*self.raw.data;

            let display = Display::new(
                self.server.clone(),
                self.index == self.default_index,
                data.width_in_pixels,
                data.height_in_pixels,
                data.root
            );

            self.index += 1;
            xcb_screen_next(&mut self.raw);
            Some(display)
        }
    }
}
