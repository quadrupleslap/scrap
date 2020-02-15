use std::env;

fn main() {
    if env::var("CARGO_CFG_WINDOWS").is_ok() {
        // The first choice is Windows because DXGI is amazing.
        println!("cargo:rustc-cfg=dxgi");
    } else if env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
        // Quartz is second because macOS is the (annoying) exception.
        println!("cargo:rustc-cfg=quartz");
    } else if env::var("CARGO_CFG_UNIX").is_ok() {
        // On UNIX we pray that X11 (with XCB) is available.
        println!("cargo:rustc-cfg=x11");
    } else {
        panic!("Platform not supported");
    }
}
