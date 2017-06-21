//TODO: Quartz.

use dxgi;
use std::io;
use PixelFormat;

pub struct Capturer {
    inner: dxgi::Capturer,
    width: usize,
    height: usize
}

impl Capturer {
    pub fn new(display: Display) -> io::Result<Capturer> {
        let width = display.width();
        let height = display.height();
        let inner = dxgi::Capturer::new(&display.inner)?;
        Ok(Capturer { inner, width, height })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn format(&self) -> PixelFormat {
        PixelFormat::Argb8888
    }

    pub fn frame<'a>(&'a mut self) -> io::Result<&'a [u8]> {
        const MILLISECONDS_PER_FRAME: u32 = 16;
        self.inner.frame(MILLISECONDS_PER_FRAME)
    }
}

pub struct Display {
    inner: dxgi::Display
}

impl Display {
    pub fn main() -> io::Result<Display> {
        match dxgi::Displays::new()?.next() {
            Some(inner) => Ok(Display { inner }),
            None => Err(io::ErrorKind::NotFound.into())
        }
    }

    pub fn all() -> io::Result<Vec<Display>> {
        Ok(dxgi::Displays::new()?
            .map(|inner| Display { inner })
            .collect::<Vec<_>>())
    }

    pub fn width(&self) -> usize {
        self.inner.width() as usize
    }

    pub fn height(&self) -> usize {
        self.inner.height() as usize
    }
}
