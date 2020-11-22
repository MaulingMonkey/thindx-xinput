use crate::*;

use winapi::shared::d3d9types::*;

use std::convert::TryInto;
use std::ptr::{null, null_mut};

// TODO: support for Device s in doc comment examples (via dev crate?)
// TODO: fuzz / torture-test Device operations in randomized combinations for odd interactions



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9)\]
/// Core interface used for general rendering, resource creation, etc.
#[derive(Clone)] #[repr(transparent)]
pub struct Device(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DDevice9>);



impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginscene)\]
    /// IDirect3DDevice9::BeginScene
    ///
    /// ### Returns
    ///
    /// *   `D3DERR::INVALIDCALL`   if the device was already within a scene (e.g. [begin_scene] was called twice without an intervening [end_scene])
    /// *   `Ok(())`                if the device was not already within a scene, and now is
    ///
    /// [begin_scene]:          #method.begin_scene
    /// [end_scene]:            #method.end_scene
    pub fn begin_scene(&self) -> Result<(), MethodError> {
        // TODO: examples
        let hr = unsafe { self.0.BeginScene() };
        MethodError::check("IDirect3DDevice9::BeginScene", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endscene)\]
    /// IDirect3DDevice9::EndScene
    ///
    /// ### Returns
    ///
    /// *   `D3DERR::INVALIDCALL`   if the device was not within a scene (e.g. [end_scene] was called without a [begin_scene], or was called a second time)
    /// *   `Ok(())`                if the device was within a scene that has now ended
    ///
    /// [begin_scene]:          #method.begin_scene
    /// [end_scene]:            #method.end_scene
    pub fn end_scene(&self) -> Result<(), MethodError> {
        // TODO: examples
        let hr = unsafe { self.0.EndScene() };
        MethodError::check("IDirect3DDevice9::EndScene", hr)
    }

    // TODO: fn scene(&self) with sane error handling / drop behavior?
    // TODO: examples
}

#[test] fn begin_end_scene() {
    let device = Device::test();
    assert_eq!(D3DERR::INVALIDCALL, device.end_scene());

    device.begin_scene().unwrap();
    device.end_scene().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.end_scene());

    device.begin_scene().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.begin_scene());
    device.end_scene().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.end_scene());

    device.begin_scene().unwrap();
    for _ in 0..1000 { assert_eq!(D3DERR::INVALIDCALL, device.begin_scene()); }
    device.end_scene().unwrap();
    for _ in 0..1000 { assert_eq!(D3DERR::INVALIDCALL, device.end_scene()); }
}



impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginstateblock)\]
    /// IDirect3DDevice9::BeginStateBlock
    ///
    /// ### Returns
    ///
    /// *   `D3DERR::INVALIDCALL`   if the device was already within a state block
    /// *   `Ok(())`                otherwise
    pub fn begin_state_block(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.0.BeginStateBlock() };
        MethodError::check("IDirect3DDevice9::BeginStateBlock", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endstateblock)\]
    /// IDirect3DDevice9::EndStateBlock
    ///
    /// ### Returns
    ///
    /// *   `D3DERR::INVALIDCALL`   if the device wasn't within a state block
    /// *   `Ok(state_block)`       if a state block was successfully captured
    pub fn end_state_block(&self) -> Result<StateBlock, MethodError> {
        let mut sb = null_mut();
        let hr = unsafe { self.0.EndStateBlock(&mut sb) };
        MethodError::check("IDirect3DDevice9::EndStateBlock", hr)?;
        Ok(unsafe { StateBlock::from_raw(sb) })
    }

    // TODO: fn state_block_scope(&self) with sane drop behavior?
    // TODO: examples
}

#[test] fn begin_end_state_block() {
    let device = Device::test();
    assert_eq!(D3DERR::INVALIDCALL, device.end_state_block().err()); // not in a state block

    device.begin_state_block().unwrap();
    device.end_state_block().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.end_state_block().err()); // not in a state block

    device.begin_state_block().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.begin_state_block().err()); // cannot nest state blocks
    device.end_state_block().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.end_state_block().err());
}



impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-clear)\]
    /// IDirect3DDevice9::Clear
    ///
    /// Clears one or more surfaces such as a render target, multiple render targets, a stencil buffer, and a depth buffer.
    ///
    /// ### Returns
    ///
    /// * `D3DERR::INVALIDCALL`     if `rects.len()` > `u32::MAX`
    /// * `D3DERR::INVALIDCALL`     if all non-`rects` parameters were `None`
    /// * `D3DERR::INVALIDCALL`     if `color`   was `Some(...)` without a render target being bound
    /// * `D3DERR::INVALIDCALL`     if `depth`   was `Some(...)` without a depth buffer being bound
    /// * `D3DERR::INVALIDCALL`     if `stencil` was `Some(...)` without a stencil buffer being bound
    /// * `Ok(())`                  otherwise
    pub fn clear(&self, rects: Option<&[Rect]>, color: Option<Color>, depth: Option<f32>, stencil: Option<u32>) -> Result<(), MethodError> {
        // TODO: more clear docs
        // TODO: conversion traits for params?

        let n : u32 = rects.map_or(0, |r| r.len()).try_into().map_err(|_| MethodError("Device::clear", D3DERR::INVALIDCALL))?;
        let rects = rects.map_or(null(), |r| r.as_ptr().cast());

        let flags =
            ((color  .is_some() as u32) * D3DCLEAR_TARGET ) |
            ((depth  .is_some() as u32) * D3DCLEAR_ZBUFFER) |
            ((stencil.is_some() as u32) * D3DCLEAR_STENCIL);

        let color   = color.unwrap_or(Color::argb(0)).into();
        let depth   = depth.unwrap_or(0.0);
        let stencil = stencil.unwrap_or(0);

        let hr = unsafe { self.0.Clear(n, rects, flags, color, depth, stencil) };
        MethodError::check("IDirect3DDevice9::Clear", hr)
    }
}

#[test] fn clear() {
    // TODO: fuzz rects harder

    for (ds_fmt,                        has_depth,  has_sten,   require ) in [
        (None,                          false,      false,      true    ),
        (Some(Format::D24S8),           true,       true,       true    ),
        (Some(Format::D24X8),           true,       false,      true    ),
        (Some(Format::D16),             true,       false,      true    ),
        (Some(Format::D15S1),           true,       true,       false   ),
        (Some(Format::D24X4S4),         true,       true,       false   ),
        (Some(Format::D16_LOCKABLE),    true,       false,      true    ),
        (Some(Format::D24FS8),          true,       true,       false   ),
        (Some(Format::D32),             true,       false,      false   ),
        (Some(Format::D32_LOCKABLE),    true,       false,      false   ),
        (Some(Format::D32F_LOCKABLE),   true,       false,      true    ),
        (Some(Format::X8_LOCKABLE),     false,      false,      false   ),
    ].iter().copied() {
        let device = Device::test_pp(|pp| {
            if let Some(ds_fmt) = ds_fmt {
                pp.EnableAutoDepthStencil = true.into();
                pp.AutoDepthStencilFormat = ds_fmt.into();
            }
        });
        if device.is_err() && !require { continue }
        let device = device.unwrap_or_else(|err| panic!("bad depth-stencil: {:?} {:?}", ds_fmt, err));

        for remove_ds in [false, true].iter().copied() {
            let autods = device.get_depth_stencil_surface().unwrap();
            if remove_ds {
                if autods.is_none() { continue }
                device.set_depth_stencil_surface(None).unwrap();
            }

            let has_depth = has_depth && !remove_ds;
            let has_sten  = has_sten  && !remove_ds;

            for (target,                        depth,                      stencil     ) in [
                (None,                          None,                       None        ), // none
                (Some(Color::argb(0xFF112233)), Some(0.5),                  Some(0xFFFF)), // all
                // color
                (Some(Color::argb(0xFF112233)), None,                       None        ),
                // depth
                (None,                          Some(-1.0),                 None        ),
                (None,                          Some(-0.5),                 None        ),
                (None,                          Some( 0.0),                 None        ),
                (None,                          Some( 0.5),                 None        ),
                (None,                          Some( 1.0),                 None        ),
                (None,                          Some( 1.5),                 None        ),
                (None,                          Some( 2.0),                 None        ),
                (None,                          Some( 2.0),                 None        ),
                (None,                          Some(f32::MIN),             None        ),
                (None,                          Some(f32::MIN_POSITIVE),    None        ),
                (None,                          Some(f32::NAN),             None        ),
                (None,                          Some(f32::INFINITY),        None        ),
                // stencil
                (None,                          None,                       Some(0)     ),
                (None,                          None,                       Some(0xFF)  ),
                (None,                          None,                       Some(!0)    ),
            ].iter().copied() {
                println!("ds_fmt={:?} target={:?} depth={:?} stencil={:?}", ds_fmt, target, depth, stencil);
                for rects in [
                    None,
                    Some(&[][..]),
                    Some(&[Rect::from([0..0,   0..0])][..]),                                    // empty rect
                    Some(&[Rect::from([0..100, 0..100])][..]),                                  // small rect
                    Some(&[Rect::from([100..10000, 100..10000])][..]),                          // out of bounds rect
                    Some(&[Rect::from([-10000..10000, -10000..10000])][..]),                    // positive dims huge rect
                    Some(&[Rect::from([10000..-10000, 10000..-10000])][..]),                    // negative dims huge rect
                    Some(&[Rect::from([0..100, 0..100]), Rect::from([0..100, 0..100])][..]),    // overlapping rects
                ].iter().copied() {
                    let result = device.clear(rects, target, depth, stencil);
                    println!("  rects: {: <70} == {:?}", format!("{:?}", rects), result);

                    if target.is_none() && depth.is_none() && stencil.is_none() {
                        assert_eq!(D3DERR::INVALIDCALL, result, "expected failure - target=depth=stencil=None - nothing to clear");
                    } else if depth.is_some() && !has_depth {
                        assert_eq!(D3DERR::INVALIDCALL, result, "expected failure - depth={:?} was specified but no depth buffer was bound", depth);
                    } else if stencil.is_some() && !has_sten {
                        assert_eq!(D3DERR::INVALIDCALL, result, "expected failure - stencil={:?} was specified but no stencil buffer was bound", stencil);
                    } else {
                        result.expect("expected success, got");
                    }
                }
            }

            device.set_depth_stencil_surface(autods.as_ref()).unwrap();
        }
    }
}



impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrendertarget)\]
    /// IDirect3DDevice9::GetRenderTarget
    ///
    /// Typically, methods that return state will not work on a device that is created using D3DCREATE_PUREDEVICE. This method however, will work even on a pure device because it returns an interface.
    /// The device can now support multiple render targets. The number of render targets supported by a device is contained in the NumSimultaneousRTs member of [Caps]. See [Multiple Render Targets (Direct3D 9)].
    ///
    /// ### Returns
    ///
    /// * `D3DERR::???`             `render_target_index` > [Caps].NumSimultaneousRTs ?
    /// * `Ok(Some(Surface))`       the render target bound to that index
    /// * `Ok(None)`                no render target was bound to that index
    ///
    /// [Multiple Render Targets (Direct3D 9)]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/multiple-render-targets
    fn get_render_target(&self, render_target_index: u32) -> Result<Option<Surface>, MethodError> {
        // TODO: verify soundness before making pub
        let mut rt = null_mut();
        let hr = unsafe { self.0.GetRenderTarget(render_target_index, &mut rt) };
        if hr == D3DERR::NOTFOUND {
            Ok(None)
        } else {
            MethodError::check("IDirect3DDevice9::GetRenderTarget", hr)?;
            Ok(unsafe { Surface::from_raw_opt(rt) })
        }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setrendertarget)\]
    /// IDirect3DDevice9::SetRenderTarget
    ///
    /// ### Returns
    ///
    /// * `D3DERR::INVALIDCALL`         if `render_target_index == 0` and `render_target == None`
    /// * `D3DERR::INVALIDCALL`         if `render_target == Some(surface)` and `surface.usage() != Usage::RenderTarget`
    /// * `Ok(())`                      if the render target was successfully bound
    fn set_render_target(&self, render_target_index: u32, render_target: Option<&Surface>) -> Result<(), MethodError> {
        let rt = render_target.map_or(null_mut(), |rt| rt.as_raw());
        let hr = unsafe { self.0.SetRenderTarget(render_target_index, rt) };
        MethodError::check("IDirect3DDevice9::SetRenderTarget", hr)
    }
}

#[test] fn get_set_render_target() {
    let device = Device::test();
    //let caps = device...
    let max_rts = 4; // TODO: caps

    // attempt to muck with RT0
    let rt0 = device.get_render_target(0).expect("RT0 inaccessable").expect("RT0 null");
    device.set_render_target(0, Some(&rt0)).unwrap();
    assert_eq!(rt0.as_raw(), device.get_render_target(0).expect("RT0 inaccessable").expect("RT0 null").as_raw());
    assert_eq!(D3DERR::INVALIDCALL, device.set_render_target(0, None));
    assert_eq!(rt0.as_raw(), device.get_render_target(0).expect("RT0 inaccessable").expect("RT0 null").as_raw());
    device.set_render_target(0, Some(&rt0)).unwrap();
    assert_eq!(rt0.as_raw(), device.get_render_target(0).expect("RT0 inaccessable").expect("RT0 null").as_raw());

    // muck with probably-in-bounds RTs
    for i in 1..max_rts {
        // TODO: create real textures to bind
        // println!("rt[{}]", i);
        // device.set_render_target(i, Some(&rt0)).unwrap();
    }

    // TODO: What happens with setting surfaces with poor formats?
    // TODO: What happens with setting surfaces with mismatched resolutions?

    // muck with probably-out-of-bounds RTs
    for i in [100, 1000, 100000, !0].iter().copied() {
        // TODO: create real textures to bind
        assert_eq!(D3DERR::INVALIDCALL, device.set_render_target(i, Some(&rt0)));
    }
}



impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdepthstencilsurface)\]
    /// IDirect3DDevice9::GetDepthStencilSurface
    ///
    /// Gets the depth-stencil surface owned by the Direct3DDevice object.
    ///
    /// ### Returns
    ///
    /// * <span style="inaccurate">`D3DERR::INVALIDCALL`     ...?</span>
    /// * `Ok(Some(Surface))`       the render target bound to that index
    /// * `Ok(None)`                no render target was bound to that index
    ///
    /// [Multiple Render Targets (Direct3D 9)]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/multiple-render-targets
    fn get_depth_stencil_surface(&self) -> Result<Option<Surface>, MethodError> {
        // TODO: verify soundness before making pub
        let mut ds = null_mut();
        let hr = unsafe { self.0.GetDepthStencilSurface(&mut ds) };
        if hr == D3DERR::NOTFOUND {
            Ok(None)
        } else {
            MethodError::check("IDirect3DDevice9::GetDepthStencilSurface", hr)?;
            Ok(unsafe { Surface::from_raw_opt(ds) })
        }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setdepthstencilsurface)\]
    /// IDirect3DDevice9::SetDepthStencilSurface
    ///
    /// ### Returns
    ///
    /// * `D3DERR::INVALIDCALL`         if `depth_stencil_surface == Some(surface)` and `surface.usage() != Usage::DepthStencil`
    /// * `Ok(())`                      if the depth stencil was successfully (un)bound
    fn set_depth_stencil_surface(&self, depth_stencil_surface: Option<&Surface>) -> Result<(), MethodError> {
        let ds = depth_stencil_surface.map_or(null_mut(), |ds| ds.as_raw());
        let hr = unsafe { self.0.SetDepthStencilSurface(ds) };
        MethodError::check("IDirect3DDevice9::SetDepthStencilSurface", hr)
    }
}

#[test] fn get_set_depth_stencil_surface() {
    let device = Device::test_pp(|pp| {
        pp.EnableAutoDepthStencil = true.into();
        pp.AutoDepthStencilFormat = Format::D24S8.into();
    }).unwrap();

    let ds = device.get_depth_stencil_surface().unwrap().unwrap();
    assert_eq!(ds.as_raw(), device.get_depth_stencil_surface().unwrap().unwrap().as_raw());
    device.set_depth_stencil_surface(Some(&ds)).unwrap();
    assert_eq!(ds.as_raw(), device.get_depth_stencil_surface().unwrap().unwrap().as_raw());
    device.set_depth_stencil_surface(None).unwrap();
    assert!(device.get_depth_stencil_surface().unwrap().is_none());

    // TODO: What happens with pure devices?
    // TODO: What happens with setting surfaces with poor formats?
    // TODO: What happens with setting surfaces with mismatched resolutions?
}



#[cfg(test)] impl Device {
    pub fn test() -> Self { Self::test_pp(|_| {}).unwrap() }

    pub fn test_pp(ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS)) -> Result<Self, MethodError> {
        // winit's new_any_thread is unix-only, but I want/need threaded windows unit tests, so create an HWND from scratch.
        use wchar::wch_c;

        use winapi::shared::d3d9::*;
        use winapi::shared::d3d9caps::*;
        use winapi::shared::d3d9types::*;
        use winapi::shared::windef::*;
        
        use winapi::um::libloaderapi::*;
        use winapi::um::winuser::*;

        thread_local! {
            static HWND : HWND = unsafe {
                let hinstance = GetModuleHandleW(null());
                assert!(!hinstance.is_null());

                let hcursor = LoadCursorW(null_mut(), IDC_ARROW);
                assert!(!hcursor.is_null());

                let wc = WNDCLASSW {
                    lpfnWndProc:    Some(DefWindowProcW),
                    hInstance:      hinstance,
                    hCursor:        hcursor,
                    lpszClassName:  wch_c!("UnitTestWndClass").as_ptr(),
                    .. std::mem::zeroed()
                };
                RegisterClassW(&wc); // might fail if previously registered
            
                let hwnd = CreateWindowExW(
                    0,
                    wch_c!("UnitTestWndClass").as_ptr(),
                    wch_c!("thin3d9 unit test").as_ptr(),
                    0, //WS_OVERLAPPEDWINDOW,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    800,
                    600,
                    null_mut(),
                    null_mut(),
                    hinstance,
                    null_mut(),
                );
                assert!(!hwnd.is_null());

                hwnd
            };
        }

        let mut pp = D3DPRESENT_PARAMETERS {
            Windowed:               true.into(),
            hDeviceWindow:          HWND.with(|hwnd| *hwnd),
            SwapEffect:             D3DSWAPEFFECT_DISCARD,
            PresentationInterval:   D3DPRESENT_INTERVAL_IMMEDIATE,
            .. unsafe { std::mem::zeroed() }
        };
        ppf(&mut pp);

        let behavior =
            //D3DCREATE_DISABLE_PRINTSCREEN | // not implemented by winapi?
            D3DCREATE_FPU_PRESERVE |
            D3DCREATE_HARDWARE_VERTEXPROCESSING |
            D3DCREATE_NOWINDOWCHANGES;

        let d3d = Direct3D::test();
        unsafe { d3d.create_device(0, DevType::HAL, null_mut(), behavior, &mut pp) }
    }
}
