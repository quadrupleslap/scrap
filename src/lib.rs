#[cfg(quartz)]
use block;
#[cfg(quartz)]
pub mod quartz;

#[cfg(x11)]
pub mod x11;

#[cfg(dxgi)]
use winapi;
#[cfg(dxgi)]
pub mod dxgi;

mod common;
pub use common::*;
