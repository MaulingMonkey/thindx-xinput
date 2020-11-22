#![windows_subsystem = "windows"]

use thin3d9::*;

use raw_window_handle::*;
use raw_window_handle::windows::*;

use winapi::shared::d3d9::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::d3d9types::*;
use winapi::shared::winerror::SUCCEEDED;

use winapi::um::debugapi::*;

use winit::dpi::*;
use winit::event::*;
use winit::event_loop::*;
use winit::window::*;

use std::ptr::*;



fn main() {
    std::panic::set_hook(Box::new(|pi| unsafe {
        eprintln!("{}", pi);
        if IsDebuggerPresent() != 0 { DebugBreak(); }
        std::process::exit(1);
    }));

    let event_loop  = EventLoop::new();
    let window      = WindowBuilder::new()
        .with_title("00-clear-winit - thin3d9 example")
        .with_inner_size(Size::Physical(PhysicalSize { width: 800, height: 600 }))
        .build(&event_loop).unwrap();
    let d3d         = unsafe { Direct3D::create(SdkVersion::default()) }.unwrap();

    let hwnd = match window.raw_window_handle() {
        RawWindowHandle::Windows(WindowsHandle { hwnd, .. }) => hwnd.cast(),
        other => panic!("Expected RawWindowHandle::Windows(...), got {:?} instead", other),
    };

    let mut pp = D3DPRESENT_PARAMETERS {
        Windowed:               true.into(),
        hDeviceWindow:          hwnd,
        SwapEffect:             D3DSWAPEFFECT_DISCARD,
        PresentationInterval:   D3DPRESENT_INTERVAL_ONE,
        .. unsafe { std::mem::zeroed() }
    };

    // https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcreate
    let behavior =
        //D3DCREATE_DISABLE_PRINTSCREEN | // not implemented by winapi?
        D3DCREATE_FPU_PRESERVE |
        D3DCREATE_HARDWARE_VERTEXPROCESSING |
        D3DCREATE_NOWINDOWCHANGES;

    let device = unsafe { d3d.create_device(0, DevType::HAL, null_mut(), behavior, &mut pp) }.unwrap();

    event_loop.run(move |event, _, control_flow|{
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, window_id } if window_id == window.id() => {
                std::process::exit(0); // device must outlast HWND which is about to be destroyed - kill the process instead
            },
            Event::MainEventsCleared => {
                device.clear(None, Some(Color::argb(0xFF224466)), None, None).unwrap();
                assert!(SUCCEEDED(unsafe { (*device.as_raw()).Present(null(), null(), null_mut(), null()) }));
            },
            _ => {},
        }
    });
}
