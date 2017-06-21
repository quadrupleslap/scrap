extern crate scrap;

fn main() {
    use scrap::{Capturer, Display};
    use std::io::Write;
    use std::process::{Command, Stdio};

    let d = Display::main().unwrap();
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
        if let Ok(frame) = capturer.frame() {
            if out.write_all(&*frame).is_err() {
                break;
            }
        }
    }
}
