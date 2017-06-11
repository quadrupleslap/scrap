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

cfg_block!(cfg(x11) {
    #[macro_use]
    extern crate ioctl_gen;
});

//TODO: Windows (DXGI)
//TODO: Linux (X11)
//TODO: Android (NO IDEA)
