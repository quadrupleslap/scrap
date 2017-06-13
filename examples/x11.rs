extern crate scrap;

#[cfg(x11)]
fn main() {
    use scrap::x11::*;
    use std::io::Write;
    use std::process::{Command, Stdio};
    use std::thread;

    let server = Server::default().unwrap();
    let display = server.displays().next().unwrap();
    let (w, h) = (display.width(), display.height());

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

    let mut capturer = Capturer::new(display).unwrap();
    let mut out = child.stdin.unwrap();

    loop {
        if out.write(capturer.frame().unwrap()).is_err() {
            return;
        }
        thread::sleep_ms(1000/60);
    }
}

#[cfg(not(x11))]
fn main() {
    println!("This example requires X11, which you don't have.");
}
