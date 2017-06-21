cfg_block!(cfg(quartz) {
    mod quartz;
    pub use self::quartz::*;
});

cfg_block!(cfg(x11) {
    mod x11;
    pub use self::x11::*;
});

cfg_block!(cfg(dxgi) {
    mod dxgi;
    pub use self::dxgi::*;
});

pub enum PixelFormat {
    /// Packed Little Endian ARGB8888
    Argb8888,
    /// Packed Little Endian ARGB2101010
    Argb2101010,
    /// 2-plane "video" range YCbCr 4:2:0
    YCbCr420Video,
    /// 2-plane "full" range YCbCr 4:2:0
    YCbCr420Full
}