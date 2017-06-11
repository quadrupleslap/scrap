fn main() {
    if cfg!(target_os="macos") {
        println!("cargo:rustc-cfg=quartz");
    } else if cfg!(target_os="linux") {
        println!("cargo:rustc-cfg=x11");
        println!("cargo:rustc-cfg=wayland");
    }
}
