fn main() {
    if cfg!(target_os="macos") {
        println!("cargo:rustc-cfg=quartz");
    }
    
    if cfg!(target_os="linux") {
        println!("cargo:rustc-cfg=x11");
    }
    
    if cfg!(target_os="windows") {
        println!("cargo:rustc-cfg=dxgi");
    }
}
