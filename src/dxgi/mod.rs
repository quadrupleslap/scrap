use self::ffi::*;
use std::{io, mem, ptr, slice};
use winapi::{
    HRESULT,
    IDXGIAdapter1,
    IDXGIFactory1,
    IDXGIOutput1,
    S_OK,
    UINT,
    DXGI_OUTPUT_DESC,
    LONG,
    DXGI_MODE_ROTATION,
    ID3D11Device,
    ID3D11DeviceContext,
    IDXGIOutputDuplication,
    D3D11_SDK_VERSION,
    D3D_DRIVER_TYPE_UNKNOWN,
    D3D_FEATURE_LEVEL_9_1,
    DXGI_ERROR_ACCESS_LOST,
    DXGI_ERROR_WAIT_TIMEOUT,
    DXGI_ERROR_INVALID_CALL,
    E_ACCESSDENIED,
    DXGI_ERROR_UNSUPPORTED,
    DXGI_ERROR_NOT_CURRENTLY_AVAILABLE,
    DXGI_ERROR_SESSION_DISCONNECTED
};

mod ffi;

//TODO: Split up into files.
//TODO: Support non-mappable outputs, too.
//TODO: Apply rotations.

pub struct Capturer {
    device: *mut ID3D11Device,
    context: *mut ID3D11DeviceContext,
    duplication: *mut IDXGIOutputDuplication,
    height: usize,
    data: *mut u8,
    len: usize
}

impl Capturer {
    pub fn new(display: &Display) -> io::Result<Capturer> {
        let mut device = ptr::null_mut();
        let mut context = ptr::null_mut();
        let mut duplication = ptr::null_mut();

        if unsafe {
            D3D11CreateDevice(
                &mut **display.adapter,
                D3D_DRIVER_TYPE_UNKNOWN,
                ptr::null_mut(), // No software rasterizer.
                0, // No device flags.
                ptr::null_mut(), // Feature levels.
                0, // Feature levels' length.
                D3D11_SDK_VERSION,
                &mut device,
                &mut D3D_FEATURE_LEVEL_9_1,
                &mut context
            )
        } != S_OK {
            // Unknown error.
            return Err(io::ErrorKind::Other.into());
        }

        wrap_hresult(unsafe {
            (*display.inner).DuplicateOutput(
                &mut **device,
                &mut duplication
            )
        })?;

        Ok(unsafe {
            let mut capturer = Capturer {
                device, context, duplication,
                height: display.height() as usize,
                data: ptr::null_mut(),
                len: 0
            };
            let _ = capturer.load_frame(0);
            capturer
        })
    }

    unsafe fn load_frame(&mut self, timeout: UINT) -> io::Result<()> {
        let mut frame = ptr::null_mut();
        let mut info = mem::uninitialized();
        let mut rect = mem::uninitialized();
        self.data = ptr::null_mut();

        wrap_hresult((*self.duplication).AcquireNextFrame(
            timeout,
            &mut info,
            &mut frame
        ))?;

        wrap_hresult((*self.duplication).MapDesktopSurface(&mut rect))?;
        
        (*frame).Release();
        self.data = rect.pBits;
        self.len = self.height * rect.Pitch as usize;

        Ok(())
    }
    
    pub fn frame<'a>(&'a mut self, timeout: UINT) -> io::Result<&'a [u8]> {
        unsafe {
            // Release last frame.
            // No error checking needed because we don't care.
            // All the errors don't crash (I think.)
            (*self.duplication).UnMapDesktopSurface();
            (*self.duplication).ReleaseFrame();

            // Get next frame.
            self.load_frame(timeout)?;
            // No error checking because all errors caught by `load_frame`.
            Ok(slice::from_raw_parts(self.data, self.len))
        }
    }
}

impl Drop for Capturer {
    fn drop(&mut self) {
        unsafe {
            (*self.duplication).Release();
            (*self.device).Release();
            (*self.context).Release();
        }
    }
}

pub struct Displays {
    factory: *mut IDXGIFactory1,
    adapter: *mut IDXGIAdapter1,
    /// Index of the CURRENT adapter.
    nadapter: UINT,
    /// Index of the NEXT display to fetch.
    ndisplay: UINT
}

impl Displays {
    pub fn new() -> io::Result<Displays> {
        let mut factory = ptr::null_mut();
        wrap_hresult(unsafe {
            CreateDXGIFactory1(&IID_IDXGIFACTORY1, &mut factory)
        })?;

        let mut adapter = ptr::null_mut();
        wrap_hresult(unsafe {
            (*factory).EnumAdapters1(0, &mut adapter)
        })?;

        Ok(Displays {
            factory,
            adapter,
            nadapter: 0,
            ndisplay: 0
        })
    }

    // No Adapter => Some(None)
    // Non-Empty Adapter => Some(Some(OUTPUT))
    // End of Adapter => None
    fn read_and_invalidate(&mut self) -> Option<Option<Display>> {
        if self.adapter.is_null() {
            return Some(None);
        }

        let adapter = unsafe { &mut *self.adapter };

        let output = unsafe {
            let mut output = ptr::null_mut();
            adapter.EnumOutputs(self.ndisplay, &mut output);
            output
        };

        if output.is_null() {
            unsafe { adapter.Release(); }
            return None;
        }

        self.ndisplay += 1;

        let desc = unsafe {
            let mut desc = mem::uninitialized();
            (*output).GetDesc(&mut desc);
            desc
        };

        let mut inner = ptr::null_mut();
        unsafe {
            // Errors guaranteed to result in null.
            (*output).QueryInterface(
                &IID_IDXGIOUTPUT1,
                &mut inner as *mut *mut _ as *mut *mut _
            );
        }

        if inner.is_null() {
            unsafe {
                adapter.Release();
                self.adapter = ptr::null_mut();
            }
            return None;
        }
        
        unsafe {
            adapter.AddRef();
        }

        Some(Some(Display { inner, adapter, desc }))
    }
}

impl Iterator for Displays {
    type Item = Display;
    fn next(&mut self) -> Option<Display> {
        if let Some(res) = self.read_and_invalidate() {
            res
        } else {
            // We need to replace the adapter.

            self.ndisplay = 0;
            self.nadapter += 1;

            self.adapter = unsafe {
                let mut adapter = ptr::null_mut();
                (*self.factory).EnumAdapters1(
                    self.nadapter,
                    &mut adapter
                );
                adapter
            };

            if let Some(res) = self.read_and_invalidate() {
                res
            } else {
                // All subsequent adapters will also be empty.
                None
            }
        }
    }
}

impl Drop for Displays {
    fn drop(&mut self) {
        unsafe {
            (*self.factory).Release();
            if !self.adapter.is_null() {
                (*self.adapter).Release();
            }
        }
    }
}

pub struct Display {
    inner: *mut IDXGIOutput1,
    adapter: *mut IDXGIAdapter1,
    desc: DXGI_OUTPUT_DESC
}

impl Display {
    pub fn width(&self) -> LONG {
        self.desc.DesktopCoordinates.right -
        self.desc.DesktopCoordinates.left
    }

    pub fn height(&self) -> LONG {
        self.desc.DesktopCoordinates.bottom -
        self.desc.DesktopCoordinates.top
    }

    pub fn rotation(&self) -> DXGI_MODE_ROTATION {
        self.desc.Rotation
    }

    pub fn name(&self) -> &[u16] {
        let s = &self.desc.DeviceName;
        let i = s.iter()
            .position(|&x| x == 0)
            .unwrap_or(s.len());
        &s[..i]
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            (*self.inner).Release();
            (*self.adapter).Release();
        }
    }
}

fn wrap_hresult(x: HRESULT) -> io::Result<()> {
    use std::io::ErrorKind::*;

    Err((match x {
        S_OK => return Ok(()),
        DXGI_ERROR_ACCESS_LOST => ConnectionReset,
        DXGI_ERROR_WAIT_TIMEOUT => TimedOut,
        DXGI_ERROR_INVALID_CALL => InvalidData,
        E_ACCESSDENIED => PermissionDenied,
        DXGI_ERROR_UNSUPPORTED => ConnectionRefused,
        DXGI_ERROR_NOT_CURRENTLY_AVAILABLE => Interrupted,
        DXGI_ERROR_SESSION_DISCONNECTED => ConnectionAborted,
        _ => Other
    }).into())
}
