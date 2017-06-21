use x11;
use std::{io, ops};
use std::rc::Rc;
use PixelFormat;

pub struct Capturer(x11::Capturer);

impl Capturer {
    pub fn new(display: Display) -> io::Result<Capturer> {
        x11::Capturer::new(display.0).map(Capturer)
    }

    pub fn width(&self) -> usize {
        self.0.display().width() as usize
    }

    pub fn height(&self) -> usize {
        self.0.display().height() as usize
    }

    pub fn format(&self) -> PixelFormat {
        PixelFormat::Argb8888
    }

    pub fn frame<'a>(&'a mut self) -> io::Result<Frame<'a>> {
        Ok(Frame(self.0.frame()))
    }
}

pub struct Frame<'a>(&'a [u8]);

impl<'a> ops::Deref for Frame<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.0
    }
}

pub struct Display(x11::Display);

impl Display {
    pub fn main() -> io::Result<Display> {
        let server = Rc::new(match x11::Server::default() {
            Ok(server) => server,
            Err(_) => return Err(io::ErrorKind::ConnectionRefused.into())
        });

        let mut displays = x11::Server::displays(server);
        let mut best = displays.next();
        if best.as_ref().map(|x| x.is_default()) == Some(false) {
            best = displays.find(|x| x.is_default()).or(best);
        }

        match best {
            Some(best) => Ok(Display(best)),
            None => Err(io::ErrorKind::NotFound.into())
        }
    }

    pub fn all() -> io::Result<Vec<Display>> {
        let server = Rc::new(match x11::Server::default() {
            Ok(server) => server,
            Err(_) => return Err(io::ErrorKind::ConnectionRefused.into())
        });

        Ok(x11::Server::displays(server).map(Display).collect())
    }

    pub fn width(&self) -> usize {
        self.0.width() as usize
    }

    pub fn height(&self) -> usize {
        self.0.height() as usize
    }
}
