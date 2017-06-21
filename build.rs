fn main() {
    if cfg!(target_os="macos") {
        println!("cargo:rustc-cfg=quartz");
    } else if cfg!(target_os="windows") {
        println!("cargo:rustc-cfg=dxgi");
    } else if cfg!(unix) {
        println!("cargo:rustc-cfg=x11");
    }
}
