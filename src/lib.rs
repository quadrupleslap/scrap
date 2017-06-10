macro_rules! cfg_block {
    ($x:meta { $($y:item)* }) => {
        $( #[$x] $y )*
    }
}

cfg_block!(cfg(quartz) {
    extern crate libc;
    extern crate block;
    pub mod quartz;
});

//TODO: Windows (DXGI)
//TODO: Linux (X11)
//TODO: Linux (Wayland)
//TODO: Android (NO IDEA)
