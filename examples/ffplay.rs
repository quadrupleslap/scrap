extern crate scrap;

fn main() {
    use scrap::{Capturer, Display};
    use std::io::Write;
    use std::io::ErrorKind::WouldBlock;
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::Duration;

    let one_frame = Duration::new(1, 0) / 60;

    let d = Display::primary().unwrap();
    let (w, h) = (d.width(), d.height());

    let child =
        Command::new("ffplay")
        .args(&[
            "-f", "rawvideo", "-pixel_format", "bgr0", "-video_size",
            &format!("{}x{}", w, h),
            "-"
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay, which you don't have.");

    let mut capturer = Capturer::new(d).unwrap();
    let mut out = child.stdin.unwrap();

    loop {
        match capturer.frame() {
            Ok(frame) => {
                // Write the frame.
                if out.write_all(&*frame).is_err() {
                    break;
                }
            }
            Err(ref e) if e.kind() == WouldBlock => {
                // We wait.
            }
            Err(_) => {
                // We're done here.
                break;
            }
        }

        thread::sleep(one_frame);
    }
}
