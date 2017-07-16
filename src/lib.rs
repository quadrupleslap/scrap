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

cfg_block!(cfg(dxgi) {
    extern crate winapi;
    pub mod dxgi;
});

mod common;
pub use common::*;
