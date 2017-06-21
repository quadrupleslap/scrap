use quartz;
use std::{io, ops};
use std::sync::{Arc, Mutex, MutexGuard};
use PixelFormat;

pub struct Capturer {
    inner: quartz::Capturer,
    //TODO: This is horrifying and hopefully unnecessary.
    frame: Arc<Mutex<Option<quartz::Frame>>>
}

impl Capturer {
    pub fn new(display: Display) -> io::Result<Capturer> {
        let frame = Arc::new(Mutex::new(None));

        let f = frame.clone();
        let inner = quartz::Capturer::new(
            display.0,
            display.width(),
            display.height(),
            quartz::PixelFormat::Argb8888,
            Default::default(),
            move |inner| {
                //TODO: Unwinds into C code.
                *f.lock().unwrap() = Some(inner);
            }
        ).map_err(|_| io::Error::from(io::ErrorKind::Other))?;

        Ok(Capturer { inner, frame })
    }

    pub fn width(&self) -> usize {
        self.inner.width()
    }

    pub fn height(&self) -> usize {
        self.inner.height()
    }

    pub fn format(&self) -> PixelFormat {
        use quartz::PixelFormat::*;
        match self.inner.format() {
            Argb8888 => PixelFormat::Argb8888,
            Argb2101010 => PixelFormat::Argb2101010,
            YCbCr420Video => PixelFormat::YCbCr420Video,
            YCbCr420Full => PixelFormat::YCbCr420Full,
            _ => PixelFormat::Other
        }
    }

    pub fn frame<'a>(&'a mut self) -> io::Result<Frame<'a>> {
        if let Ok(frame) = self.frame.lock() {
            if frame.is_some() {
                return Ok(Frame(frame));
            }
        }
        Err(io::ErrorKind::Other.into())
    }
}

pub struct Frame<'a>(MutexGuard<'a, Option<quartz::Frame>>);

impl<'a> ops::Deref for Frame<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        // Verified to be Some during construction.
        &*(&*self.0).as_ref().unwrap()
    }
}

pub struct Display(quartz::Display);

impl Display {
    pub fn main() -> io::Result<Display> {
        Ok(Display(quartz::Display::main()))
    }

    pub fn all() -> io::Result<Vec<Display>> {
        Ok(
            quartz::Display::online()
                .map_err(|_| io::Error::from(io::ErrorKind::Other))?
                .into_iter()
                .map(Display)
                .collect()
        )
    }

    pub fn width(&self) -> usize {
        self.0.width()
    }

    pub fn height(&self) -> usize {
        self.0.height()
    }
}
