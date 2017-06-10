extern crate scrap;

#[cfg(quartz)]
fn main() {
    use scrap::quartz::{Capturer, Display, PixelFormat, Config};
    use std::io::Write;
    use std::process::{Command, Stdio};
    use std::sync::Mutex;
    use std::thread;

    let d = Display::main();
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

    let out = Mutex::new(child.stdin.unwrap());
    let main = thread::current();

    let c = Capturer::new(
        d, w, h, PixelFormat::Argb8888,
        Config {
            cursor: false,
            letterbox: true,
            throttle: 0.0,
            queue_length: 2
        },
        move |frame| {
            if let Ok(mut out) = out.try_lock() {
                if out.write_all(&*frame).is_err() {
                    main.unpark();
                }
            }
        }
    );

    thread::park();
    drop(c);
}

#[cfg(not(quartz))]
fn main() {
    println!("This example requires Quartz, which you don't have. (.︿.)");
}
