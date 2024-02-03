#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)] // fn names

use crate::*;

use minidl::Library;

use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::*;
use winapi::um::winnt::*;

use std::os::windows::ffi::OsStrExt;
use std::sync::Once;

use core::convert::*;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::*;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, Ordering, Ordering::Relaxed};

type XINPUT_BATTERY_INFORMATION = crate::BatteryInformation;
type XINPUT_CAPABILITIES        = crate::Capabilities;
type XINPUT_KEYSTROKE           = crate::Keystroke;
type XINPUT_STATE               = crate::State;
type XINPUT_VIBRATION           = crate::Vibration;
type GUID                       = crate::DSoundAudioDeviceGuid;



// Official Imports

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate)\]
/// Get gamepad button state.
///
/// | XInput | State    |
/// | ------ | -------- |
/// | \*    | Available |
pub(crate) static XInputGetState: AtomicFn<unsafe extern "system" fn(dwUserIndex: DWORD, pState: *mut XINPUT_STATE) -> DWORD> = AtomicFn::new(lazy::XInputGetState);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputsetstate)\]
/// Set gamepad vibration state.
///
/// | XInput | State    |
/// | ------ | -------- |
/// | \*    | Available |
pub(crate) static XInputSetState: AtomicFn<unsafe extern "system" fn(dwUserIndex: DWORD, pVibration: *mut XINPUT_VIBRATION) -> DWORD> = AtomicFn::new(lazy::XInputSetState);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities)\]
/// Query the capabilities of a gamepad (vibration, wireless, voice, etc.)
///
/// | XInput | State    |
/// | ------ | -------- |
/// | \*    | Available |
pub(crate) static XInputGetCapabilities: AtomicFn<unsafe extern "system" fn(dwUserIndex: DWORD, dwFlags: DWORD, pCapabilities: *mut XINPUT_CAPABILITIES) -> DWORD> = AtomicFn::new(lazy::XInputGetCapabilities);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetdsoundaudiodeviceguids)]
/// Get DirectSound Audio Device GUIDs (N/A for Windows Store apps).
///
/// | XInput | State    |
/// | ------ | -------- |
/// | Uap   | N/A       |
/// | 1.4   | N/A       |
/// | 1.3   | Available |
/// | 1.2   | Available |
/// | 1.1   | Available |
/// | 9.1.0 | Available |
pub(crate) static XInputGetDSoundAudioDeviceGuids: AtomicFn<unsafe extern "system" fn(dwUserIndex: DWORD, pDSoundRenderGuid: *mut GUID, pDSoundCaptureGuid: *mut GUID) -> DWORD> = AtomicFn::new(lazy::XInputGetDSoundAudioDeviceGuids);

// Windows 8+

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetaudiodeviceids)]
/// Get XAudio2 Device Names.
///
/// | XInput | State    |
/// | ------ | -------- |
/// | Uap   | Available? |
/// | 1.4   | Available |
/// | 1.3   | N/A       |
/// | 1.2   | N/A       |
/// | 1.1   | N/A       |
/// | 9.1.0 | N/A       |
pub(crate) static XInputGetAudioDeviceIds: AtomicFn<unsafe extern "system" fn(dwUserIndex: DWORD, pRenderDeviceId: LPWSTR, pRenderCount: *mut UINT, pCaptureDeviceId: LPWSTR, pCaptureCount: *mut UINT) -> DWORD> = AtomicFn::new(lazy::XInputGetAudioDeviceIds);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputenable)]
/// Enable or disable xinput (for use in window focus/blur events.)
///
/// | XInput | State        |
/// | ------ | ------------ |
/// | UWP   | Deprecated?   |
/// | 1.4   | Available     |
/// | 1.3   | Available     |
/// | 1.2   | Available     |
/// | 1.1   | Available     |
/// | 9.1.0 | N/A           |
pub(crate) static XInputEnable: AtomicFn<unsafe extern "system" fn(enable: BOOL)> = AtomicFn::new(lazy::XInputEnable);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation)]
/// Get battery information for a wireless gamepad.
///
/// | XInput | State        |
/// | ------ | ------------ |
/// | Uap   | Available     |
/// | 1.4   | Available     |
/// | 1.3   | Available     |
/// | 1.2   | N/A           |
/// | 1.1   | N/A           |
/// | 9.1.0 | N/A           |
pub(crate) static XInputGetBatteryInformation: AtomicFn<unsafe extern "system" fn(dwUserIndex: DWORD, devType: BYTE, pBatteryInformation: *mut XINPUT_BATTERY_INFORMATION) -> DWORD> = AtomicFn::new(lazy::XInputGetBatteryInformation);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetkeystroke)]
///
/// | XInput | State        |
/// | ------ | ------------ |
/// | Uap   | Available     |
/// | 1.4   | Available     |
/// | 1.3   | Available     |
/// | 1.2   | N/A           |
/// | 1.1   | N/A           |
/// | 9.1.0 | N/A           |
pub(crate) static XInputGetKeystroke: AtomicFn<unsafe extern "system" fn(dwUserIndex: u32, dwReserved: u32, pKeystroke: *mut XINPUT_KEYSTROKE) -> DWORD> = AtomicFn::new(lazy::XInputGetKeystroke);



// Super Secret Shady Ordinal-Only Imports
// https://gist.github.com/robindegen/9446175
// https://www.reddit.com/r/ReverseEngineering/comments/148faa/xbox_360_controller_on_windows/c7ayith/

// Ordinals 100-103, available as of XInput1_3.dll, N/A as of XInputUap.dll
pub(crate) static _XInputGetStateEx:                AtomicFn<unsafe extern "system" fn(dwUserIndex: DWORD, pState: *mut XINPUT_STATE) -> DWORD> = AtomicFn::new(lazy::_XInputGetStateEx);
pub(crate) static _XInputWaitForGuideButton:        AtomicFn<unsafe extern "system" fn(dwUserIndex: DWORD, dwFlag: DWORD, pUnknown: *mut c_void) -> DWORD> = AtomicFn::new(lazy::_XInputWaitForGuideButton);
pub(crate) static _XInputCancelGuideButtonWait:     AtomicFn<unsafe extern "system" fn(dwUserIndex: DWORD) -> DWORD> = AtomicFn::new(lazy::_XInputCancelGuideButtonWait);
pub(crate) static _XInputPowerOffController:        AtomicFn<unsafe extern "system" fn(dwUserIndex: DWORD) -> DWORD> = AtomicFn::new(lazy::_XInputPowerOffController);

// Ordinals 104 / 108, available as of XInput1_4.dll, N/A as of XInputUap.dll
// _XInputGetBaseBusInformation (Ordinal 104)
// _XInputGetCapabilitiesEx     (Ordinal 108)



const THINDX_XINPUT_PANIC_MESSAGE : &'static str = "invalid value for `%THINDX_XINPUT%`: expected one of `1.4`, `1.3`, `1.2`, `1.1`, `9.1`, `9.1.0`, or `uap`";

const THINDX_XINPUT_DLL_BUILD_TIME : Option<&'static str> = match option_env!("THINDX_XINPUT") {
    None => None,
    Some(ver) => Some(thindx_xinput_var_to_dll_name(ver)),
};

const fn thindx_xinput_var_to_dll_name(thindx_xinput: &str) -> &'static str {
    match thindx_xinput.as_bytes() {
        b"uap"                                  => "XInputUap.dll",
        b"1.4" | b"1_4"                         => "XInput1_4.dll",
        b"1.3" | b"1_3"                         => "xinput1_3.dll",
        b"1.2" | b"1_2"                         => "xinput1_2.dll",
        b"1.1" | b"1_1"                         => "xinput1_1.dll",
        b"9.1" | b"9_1" | b"9.1.0" | b"9_1_0"   => "XInput9_1_0.dll",
        _                                       => panic!("{}", THINDX_XINPUT_PANIC_MESSAGE),
    }
}

fn init() {
    static ONCE : Once = Once::new();
    ONCE.call_once(||{
        // SAFETY: ⚠️ Technically unsound
        //  * We assume `hmodule`, if retrieved, is a valid xinput DLL.
        //  * We assume specific magic ordinals always map to specific un-named functions.
        unsafe {
            let xinput_env_dll = std::env::var_os("THINDX_XINPUT").map(|env| {
                let env = env.to_str().unwrap_or_else(|| panic!("{}", THINDX_XINPUT_PANIC_MESSAGE));
                thindx_xinput_var_to_dll_name(env)
            }).or(THINDX_XINPUT_DLL_BUILD_TIME);

            let known_dlls = &[
                "XInput1_4.dll",
                "xinput1_3.dll",
                "xinput1_2.dll",
                "xinput1_1.dll",
                "XInput9_1_0.dll",
                "XInputUap.dll",    // absolute last resort, breaks shutdown in non-uwp/uap desktop apps
            ];

            let lib = xinput_env_dll.and_then(|dll| Library::load(dll).ok());                       // use specified xinput first
            let lib = lib.or_else(|| known_dlls.iter().find_map(|dll| library_get(dll)));           // search already loaded XInputs
            let lib = lib.or_else(|| known_dlls.iter().find_map(|dll| Library::load(dll).ok()));    // search not yet loaded XInputs last

            let get_state : unsafe extern "system" fn(dwUserIndex: DWORD, pState: *mut XINPUT_STATE) -> DWORD = lib.and_then(|lib| lib.sym_opt("XInputGetState\0")).unwrap_or(fallback::XInputGetState);

            XInputGetState                  .store(get_state,                                                                                                                        Relaxed);
            XInputSetState                  .store(lib.and_then(|lib| lib.sym_opt("XInputSetState\0"                     )).unwrap_or(fallback::XInputSetState                    ), Relaxed);
            XInputGetCapabilities           .store(lib.and_then(|lib| lib.sym_opt("XInputGetCapabilities\0"              )).unwrap_or(fallback::XInputGetCapabilities             ), Relaxed);
            XInputGetDSoundAudioDeviceGuids .store(lib.and_then(|lib| lib.sym_opt("XInputGetDSoundAudioDeviceGuids\0"    )).unwrap_or(fallback::XInputGetDSoundAudioDeviceGuids   ), Relaxed);
            XInputGetAudioDeviceIds         .store(lib.and_then(|lib| lib.sym_opt("XInputGetAudioDeviceIds\0"            )).unwrap_or(fallback::XInputGetAudioDeviceIds           ), Relaxed);
            XInputEnable                    .store(lib.and_then(|lib| lib.sym_opt("XInputEnable\0"                       )).unwrap_or(fallback::XInputEnable                      ), Relaxed);
            XInputGetBatteryInformation     .store(lib.and_then(|lib| lib.sym_opt("XInputGetBatteryInformation\0"        )).unwrap_or(fallback::XInputGetBatteryInformation       ), Relaxed);
            XInputGetKeystroke              .store(lib.and_then(|lib| lib.sym_opt("XInputGetKeystroke\0"                 )).unwrap_or(fallback::XInputGetKeystroke                ), Relaxed);

            _XInputGetStateEx               .store(lib.and_then(|lib| lib.sym_opt_by_ordinal(100                         )).unwrap_or(get_state                                   ), Relaxed);
            _XInputWaitForGuideButton       .store(lib.and_then(|lib| lib.sym_opt_by_ordinal(101                         )).unwrap_or(fallback::_XInputWaitForGuideButton         ), Relaxed);
            _XInputCancelGuideButtonWait    .store(lib.and_then(|lib| lib.sym_opt_by_ordinal(102                         )).unwrap_or(fallback::_XInputCancelGuideButtonWait      ), Relaxed);
            _XInputPowerOffController       .store(lib.and_then(|lib| lib.sym_opt_by_ordinal(103                         )).unwrap_or(fallback::_XInputPowerOffController         ), Relaxed);

            // I don't have type information for these... yet
            //InputGetBaseBusInformation    .store(lib.and_then(|lib| lib.sym_opt_by_ordinal(104                         )).unwrap_or(fallback::_XInputGetBaseBusInformation      ), Relaxed);
            //InputGetCapabilitiesEx        .store(lib.and_then(|lib| lib.sym_opt_by_ordinal(108                         )).unwrap_or(fallback::_XInputGetCapabilitiesEx          ), Relaxed);
        }
    });
}

mod fallback {
    #![allow(non_snake_case)]
    #![allow(unused_variables)]
    use super::*;
    const ERROR_INVALID_FUNCTION : u32 = error::INVALID_FUNCTION.to_u32();

    pub extern "system" fn XInputGetState(                  dwUserIndex: DWORD, pState: *mut XINPUT_STATE                                                                               ) -> DWORD { ERROR_INVALID_FUNCTION }
    pub extern "system" fn XInputSetState(                  dwUserIndex: DWORD, pVibration: *mut XINPUT_VIBRATION                                                                       ) -> DWORD { ERROR_INVALID_FUNCTION }
    pub extern "system" fn XInputGetCapabilities(           dwUserIndex: DWORD, dwFlags: DWORD, pCapabilities: *mut XINPUT_CAPABILITIES                                                 ) -> DWORD { ERROR_INVALID_FUNCTION }
    pub extern "system" fn XInputGetDSoundAudioDeviceGuids( dwUserIndex: DWORD, pDSoundRenderGuid: *mut GUID, pDSoundCaptureGuid: *mut GUID                                             ) -> DWORD { ERROR_INVALID_FUNCTION }
    pub extern "system" fn XInputGetAudioDeviceIds(         dwUserIndex: DWORD, pRenderDeviceId: LPWSTR, pRenderCount: *mut UINT, pCaptureDeviceId: LPWSTR, pCaptureCount: *mut UINT    ) -> DWORD { ERROR_INVALID_FUNCTION }
    pub extern "system" fn XInputEnable(                    enable: BOOL                                                                                                                ) {}
    pub extern "system" fn XInputGetBatteryInformation(     dwUserIndex: DWORD, devType: BYTE, pBatteryInformation: *mut XINPUT_BATTERY_INFORMATION                                     ) -> DWORD { ERROR_INVALID_FUNCTION }
    pub extern "system" fn XInputGetKeystroke(              dwUserIndex: u32, dwReserved: u32, pKeystroke: *mut XINPUT_KEYSTROKE                                                        ) -> DWORD { ERROR_INVALID_FUNCTION }

    //b extern "system" fn _XInputGetStateEx(               dwUserIndex: DWORD, pState: *mut XINPUT_STATE                                                                               ) -> DWORD { ERROR_INVALID_FUNCTION }
    pub extern "system" fn _XInputWaitForGuideButton(       dwUserIndex: DWORD, dwFlag: DWORD, pUnknown: *mut c_void                                                                    ) -> DWORD { ERROR_INVALID_FUNCTION }
    pub extern "system" fn _XInputCancelGuideButtonWait(    dwUserIndex: DWORD                                                                                                          ) -> DWORD { ERROR_INVALID_FUNCTION }
    pub extern "system" fn _XInputPowerOffController(       dwUserIndex: DWORD                                                                                                          ) -> DWORD { ERROR_INVALID_FUNCTION }
}

mod lazy {
    use super::*;

    pub unsafe extern "system" fn XInputGetState(                  dwUserIndex: DWORD, pState: *mut XINPUT_STATE                                                                               ) -> DWORD { super::init(); unsafe { super::XInputGetState.load(Relaxed)(dwUserIndex, pState) } }
    pub unsafe extern "system" fn XInputSetState(                  dwUserIndex: DWORD, pVibration: *mut XINPUT_VIBRATION                                                                       ) -> DWORD { super::init(); unsafe { super::XInputSetState.load(Relaxed)(dwUserIndex, pVibration) } }
    pub unsafe extern "system" fn XInputGetCapabilities(           dwUserIndex: DWORD, dwFlags: DWORD, pCapabilities: *mut XINPUT_CAPABILITIES                                                 ) -> DWORD { super::init(); unsafe { super::XInputGetCapabilities.load(Relaxed)(dwUserIndex, dwFlags, pCapabilities) } }
    pub unsafe extern "system" fn XInputGetDSoundAudioDeviceGuids( dwUserIndex: DWORD, pDSoundRenderGuid: *mut GUID, pDSoundCaptureGuid: *mut GUID                                             ) -> DWORD { super::init(); unsafe { super::XInputGetDSoundAudioDeviceGuids.load(Relaxed)(dwUserIndex, pDSoundRenderGuid, pDSoundCaptureGuid) } }
    pub unsafe extern "system" fn XInputGetAudioDeviceIds(         dwUserIndex: DWORD, pRenderDeviceId: LPWSTR, pRenderCount: *mut UINT, pCaptureDeviceId: LPWSTR, pCaptureCount: *mut UINT    ) -> DWORD { super::init(); unsafe { super::XInputGetAudioDeviceIds.load(Relaxed)(dwUserIndex, pRenderDeviceId, pRenderCount, pCaptureDeviceId, pCaptureCount) } }
    pub unsafe extern "system" fn XInputEnable(                    enable: BOOL                                                                                                                )          { super::init(); unsafe { super::XInputEnable.load(Relaxed)(enable) } }
    pub unsafe extern "system" fn XInputGetBatteryInformation(     dwUserIndex: DWORD, devType: BYTE, pBatteryInformation: *mut XINPUT_BATTERY_INFORMATION                                     ) -> DWORD { super::init(); unsafe { super::XInputGetBatteryInformation.load(Relaxed)(dwUserIndex, devType, pBatteryInformation) } }
    pub unsafe extern "system" fn XInputGetKeystroke(              dwUserIndex: u32, dwReserved: u32, pKeystroke: *mut XINPUT_KEYSTROKE                                                        ) -> DWORD { super::init(); unsafe { super::XInputGetKeystroke.load(Relaxed)(dwUserIndex, dwReserved, pKeystroke) } }

    pub unsafe extern "system" fn _XInputGetStateEx(               dwUserIndex: DWORD, pState: *mut XINPUT_STATE                                                                               ) -> DWORD { super::init(); unsafe { super::_XInputGetStateEx.load(Relaxed)(dwUserIndex, pState) } }
    pub unsafe extern "system" fn _XInputWaitForGuideButton(       dwUserIndex: DWORD, dwFlag: DWORD, pUnknown: *mut c_void                                                                    ) -> DWORD { super::init(); unsafe { super::_XInputWaitForGuideButton.load(Relaxed)(dwUserIndex, dwFlag, pUnknown) } }
    pub unsafe extern "system" fn _XInputCancelGuideButtonWait(    dwUserIndex: DWORD                                                                                                          ) -> DWORD { super::init(); unsafe { super::_XInputCancelGuideButtonWait.load(Relaxed)(dwUserIndex) } }
    pub unsafe extern "system" fn _XInputPowerOffController(       dwUserIndex: DWORD                                                                                                          ) -> DWORD { super::init(); unsafe { super::_XInputPowerOffController.load(Relaxed)(dwUserIndex) } }
}



fn library_get(name: impl AsRef<std::ffi::OsStr>) -> Option<Library> {
    let name = name.as_ref();
    let name = name.encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let mut hmodule = null_mut();
    if 0 == unsafe { GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_PIN, name.as_ptr(), &mut hmodule) } { return None }
    unsafe { Library::from_ptr(hmodule.cast()) }
}

pub(crate) struct AtomicFn<F: Copy>(AtomicPtr<c_void>, PhantomData<F>);
impl<F: Copy> AtomicFn<F> {
    pub const fn new(f: F) -> Self { Self(AtomicPtr::new(unsafe { const_transmute_copy(f) }), PhantomData) }
    pub fn load(&self, order: Ordering) -> F { let p = self.0.load(order); unsafe { const_transmute_copy(p) } }
    pub fn store(&self, value: F, order: Ordering) { self.0.store(unsafe { const_transmute_copy(value) }, order) }
}

/// Same requirements as [`core::mem::transmute`]... probably!
/// Alignment/size validation is deferred until... monomorphization?
const unsafe fn const_transmute_copy<S: Copy, D: Copy>(src: S) -> D {
    struct Assert<S, D>(PhantomData<(S, D)>);
    impl<S, D> Assert<S, D> { const SAME_LAYOUT : () = assert!(align_of::<S>() == align_of::<D>() && size_of::<S>() == size_of::<D>()); }
    let _ = Assert::<S, D>::SAME_LAYOUT;
    union U<S: Copy, D: Copy> { src: S, dst: D }
    let u = U::<S, D> { src };
    unsafe { u.dst }
}
