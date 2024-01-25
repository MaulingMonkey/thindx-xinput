#![allow(non_snake_case, non_upper_case_globals)] // fn names

use crate::*;

use minidl::Library;

use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::*;
use winapi::um::processthreadsapi::*;
use winapi::um::psapi::*;
use winapi::um::winnt::*;
use winapi::um::xinput::*;

use std::sync::Once;

use core::convert::*;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::*;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, Ordering, Ordering::Relaxed};



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
pub(crate) static XInputGetKeystroke: AtomicFn<unsafe extern "system" fn(dwUserIndex: u32, dwReserved: u32, pKeystroke: PXINPUT_KEYSTROKE) -> DWORD> = AtomicFn::new(lazy::XInputGetKeystroke);



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

            let lib =
                xinput_env_dll.and_then(|dll| Library::load(dll).ok())
                .or_else(|| try_find_loaded_xinput())
                .or_else(|| Library::load("XInput1_4.dll").ok())
                .or_else(|| Library::load("xinput1_3.dll").ok())
                .or_else(|| Library::load("xinput1_2.dll").ok())
                .or_else(|| Library::load("xinput1_1.dll").ok())
                .or_else(|| Library::load("XInput9_1_0.dll").ok())
                .or_else(|| Library::load("XInputUap.dll").ok())    // absolute last resort, breaks shutdown in non-uwp/uap desktop apps
                ;

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
    pub extern "system" fn XInputGetKeystroke(              dwUserIndex: u32, dwReserved: u32, pKeystroke: PXINPUT_KEYSTROKE                                                            ) -> DWORD { ERROR_INVALID_FUNCTION }

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
    pub unsafe extern "system" fn XInputGetKeystroke(              dwUserIndex: u32, dwReserved: u32, pKeystroke: PXINPUT_KEYSTROKE                                                            ) -> DWORD { super::init(); unsafe { super::XInputGetKeystroke.load(Relaxed)(dwUserIndex, dwReserved, pKeystroke) } }

    pub unsafe extern "system" fn _XInputGetStateEx(               dwUserIndex: DWORD, pState: *mut XINPUT_STATE                                                                               ) -> DWORD { super::init(); unsafe { super::_XInputGetStateEx.load(Relaxed)(dwUserIndex, pState) } }
    pub unsafe extern "system" fn _XInputWaitForGuideButton(       dwUserIndex: DWORD, dwFlag: DWORD, pUnknown: *mut c_void                                                                    ) -> DWORD { super::init(); unsafe { super::_XInputWaitForGuideButton.load(Relaxed)(dwUserIndex, dwFlag, pUnknown) } }
    pub unsafe extern "system" fn _XInputCancelGuideButtonWait(    dwUserIndex: DWORD                                                                                                          ) -> DWORD { super::init(); unsafe { super::_XInputCancelGuideButtonWait.load(Relaxed)(dwUserIndex) } }
    pub unsafe extern "system" fn _XInputPowerOffController(       dwUserIndex: DWORD                                                                                                          ) -> DWORD { super::init(); unsafe { super::_XInputPowerOffController.load(Relaxed)(dwUserIndex) } }
}



/// Tries to find the most XInput-y looking, already loaded, DLL:
/// *   DLLs that don't export `XInputGetState` will be straight up ignored.
/// *   DLLs with more characters matching the `xinput_` prefix will be prefered.
/// *   DLLs with shorter filenames will be prefered (e.g. `XInput1_4.dll` wins out over `xinput_9_0_3.dll`)
///
/// ### ⚠️ Safety ⚠️
/// Microsoft's PSAPI documentation makes it clear that some of the stuff this relies on for e.g. process module enumeration
/// are best effort debug functionality, not battle tested production quality tooling:
///
/// > "The EnumProcessModulesEx function is primarily designed for use by debuggers and similar applications that must
/// > extract module information from another process. If the module list in the target process is corrupted or not
/// > yet initialized, or if the module list changes during the function call as a result of DLLs being loaded or
/// > unloaded, EnumProcessModulesEx may fail or return incorrect information."
/// >
/// > <https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex>
///
/// Additionally, there's technically nothing stopping you from loading an evil `xinput_.dll`, that takes priority over
/// the real xinput DLLs, that defines XInputGetState and exposes unsound functions, which immediately invokes undefined
/// behavior if you call into it.  Which means relying on the returned HMODULE to do just about anything is, *technically*,
/// unsound.
///
/// Well, perhaps eventually I'll verify xinput.dll is code-signed by Microsoft, which would fix that well enough for my
/// tastes, but for now this is good enough for me ;)
unsafe fn try_find_loaded_xinput() -> Option<Library> {
    let proc = unsafe { GetCurrentProcess() };
    let mut modules = Vec::<HMODULE>::new();

    let mut max_retries = 64;

    loop {
        let available_bytes = u32::try_from(std::mem::size_of_val(&modules[..])).unwrap_or(!0);
        let mut needed_bytes : u32 = 0;
        let ok = unsafe { EnumProcessModulesEx(proc, modules.as_mut_ptr(), available_bytes, &mut needed_bytes, LIST_MODULES_DEFAULT) };
        if ok == FALSE {
            if max_retries == 0 { return None; }
            max_retries -= 1;
            continue; // temporary failure? retry!
        }
        let needed_elements = usize::try_from(needed_bytes).unwrap_or(!0usize) / size_of::<HMODULE>();
        if needed_bytes <= available_bytes {
            modules.truncate(needed_elements);
            break // success!
        } else {
            modules.resize(needed_elements, null_mut());
            modules.resize(modules.capacity(), null_mut());
            continue // not enough modules
        }
    }

    // SAFETY: ⚠️ `m` should be a permanently loaded library... probably...
    modules.retain(|&m| unsafe { Library::from_ptr(m.cast()) }.map_or(false, |m| m.has_sym("XInputGetState\0")));

    let hmodule = match modules[..] {
        [] => None,
        [module] => Some(module),
        ref mut multiple => {
            let mut name = [0u8; 4096];
            multiple.sort_by_cached_key(|&m|{
                let len = unsafe { GetModuleBaseNameA(proc, m, name.as_mut_ptr().cast(), name.len() as _) } as usize;
                let name = &mut name[..len];
                name.make_ascii_lowercase();
                let prefix = b"xinput_";
                let matching = prefix.iter().copied().zip(name.iter().copied()).position(|(x,y)| x != y).unwrap_or(prefix.len());
                (matching * 1000 + 1000).saturating_sub(name.len()) // prioritize prefix matching "xinput_", then secondarilly prioritize shorter names.
            });
            multiple.last().copied()
        },
    };

    // SAFETY: ✔️ `hmodule` should be a valid HMODULE
    hmodule.and_then(|hmodule| unsafe { Library::from_ptr(hmodule.cast()) })
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
