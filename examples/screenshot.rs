extern crate image;
extern crate scrap;

use image::{ImageBuffer, Rgb};
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;
    let path = Path::new("screenshot.png");

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    for i in 0..3 {
        println!("{}...", 3 - i);
        thread::sleep(one_second);
    }

    loop {
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Capture error: {}", error);
                }
            }
        };

        println!("Captured! Saving...");

        // PistonDevelopers/image doesn't support ARGB images yet.
        // But they will soon!

        let mut bitflipped = Vec::with_capacity(w * h * 3);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                ]);
            }
        }

        let image: ImageBuffer<Rgb<u8>, _> =
            ImageBuffer::from_raw(
                w as u32,
                h as u32,
                bitflipped
            ).expect("Couldn't convert frame into image buffer.");

        image.save(&path).expect("Couldn't save image to `screenshot.png`.");
        println!("Image saved to `screenshot.png`.");
        break;
    }
}
