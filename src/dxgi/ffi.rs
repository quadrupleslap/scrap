use winapi::{
    GUID,
    HRESULT,
    REFIID,
    IDXGIFactory1,
    IDXGIAdapter,
    D3D_DRIVER_TYPE,
    HMODULE,
    UINT,
    ID3D11Device,
    D3D_FEATURE_LEVEL,
    ID3D11DeviceContext
};

pub const IID_IDXGIFACTORY1: GUID = GUID {
    Data1: 0x770aae78,
    Data2: 0xf26f,
    Data3: 0x4dba,
    Data4: [0xa8, 0x29, 0x25, 0x3c, 0x83, 0xd1, 0xb3, 0x87]
};

pub const IID_IDXGIOUTPUT1: GUID = GUID {
    Data1: 0x00cddea8,
    Data2: 0x939b,
    Data3: 0x4b83,
    Data4: [0xa3, 0x40, 0xa6, 0x85, 0x22, 0x66, 0x66, 0xcc]
};

#[link(name="dxgi")]
#[link(name="d3d11")]
extern "system" {
    pub fn CreateDXGIFactory1(
        id: REFIID,
        pp_factory: *mut *mut IDXGIFactory1
    ) -> HRESULT;

    pub fn D3D11CreateDevice(
        pAdapter: *mut IDXGIAdapter,
        DriverType: D3D_DRIVER_TYPE,
        Software: HMODULE,
        Flags: UINT,
        pFeatureLevels: *mut D3D_FEATURE_LEVEL,
        FeatureLevels: UINT,
        SDKVersion: UINT,
        ppDevice: *mut *mut ID3D11Device,
        pFeatureLevel: *mut D3D_FEATURE_LEVEL,
        ppImmediateContext: *mut *mut ID3D11DeviceContext
    ) -> HRESULT;
}
