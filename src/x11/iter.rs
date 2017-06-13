use super::{Display, Server};
use super::ffi::*;

pub struct DisplayIter<'a> {
    raw: xcb_screen_iterator_t,
    index: i32,
    default_index: i32,
    server: &'a Server
}

impl<'a> DisplayIter<'a> {
    pub unsafe fn new(
        raw: xcb_screen_iterator_t,
        default_index: i32,
        server: &'a Server
    ) -> DisplayIter<'a> {
        DisplayIter { raw, index: 0, default_index, server }
    }
}

impl<'a> Iterator for DisplayIter<'a> {
    type Item = Display<'a>;

    fn next(&mut self) -> Option<Display<'a>> {
        if self.raw.rem == 0 {
            return None;
        }

        unsafe {
            let data = &*self.raw.data;

            let display = Display::new(
                self.server,
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
