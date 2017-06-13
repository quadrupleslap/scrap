extern crate libc;

macro_rules! cfg_block {
    ($x:meta { $($y:item)* }) => {
        $( #[$x] $y )*
    }
}

cfg_block!(cfg(quartz) {
    extern crate block;
    pub mod quartz;
});

cfg_block!(cfg(x11) {
    pub mod x11;
});

//TODO: Windows (DXGI)
//TODO: Linux (X11)
//TODO: Android (NO IDEA)
