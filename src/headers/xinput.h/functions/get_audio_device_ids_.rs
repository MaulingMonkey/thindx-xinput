use crate::*;

use std::ffi::OsString;
use std::os::windows::ffi::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetaudiodeviceids)\]
/// XInputGetAudioDeviceIds
/// <span style="opacity: 50%">(1.4+)</span>
///
/// Get XAudio2 / Windows Core Audio Device Names.
///
/// **NOTE:** This tends to succeed, even when no gamepad is connected, with empty/None paths.
///
/// ### Errors
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - **Unreliably.**
/// *   [ERROR::INVALID_FUNCTION]       - XAudio2 / Windows Core Audio Device Names unavailable: XInput 1.4 or later
/// *   [ERROR::BUFFER_TOO_SMALL]       - Audio device paths exceedingly large (doesn't fit in e.g. `[wchar_t; 4096]`.)
///
/// | System            | Windows `ver`     | Windows SKU           | Behavior |
/// | ----------------- | ----------------- | --------------------- | -------- |
/// | Github Actions    | 10.0.17763.2366   | Windows 2019 Server   | [ERROR::DEVICE_NOT_CONNECTED] observed.
/// | "SACRILEGE"       | 10.0.19041.1415   | Windows 10 Pro        | Succeeds when called on missing gamepads
///
/// ### See Also
/// *   [Getting Audio Device Identifiers](https://learn.microsoft.com/en-us/windows/win32/xinput/getting-started-with-xinput#getting-audio-device-identifiers)
pub fn get_audio_device_ids(user_index: impl Into<u32>) -> Result<AudioDeviceIds, Error> {
    fn_context!(xinput::get_audio_device_ids => XInputGetAudioDeviceIds);
    #[allow(non_snake_case)] let XInputGetAudioDeviceIds = Imports::get().XInputGetAudioDeviceIds.ok_or(fn_error!(ERROR::INVALID_FUNCTION))?;

    let mut render_id  = [0u16; 4096];
    let mut capture_id = [0u16; 4096];
    let mut render_len  = 4096;
    let mut capture_len = 4096;

    // SAFETY: ⚠️ Needs testing with real audio devices
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `user_index`  is well tested
    //  * `*_ptr`       is never null, should only be accessed during XInputGetAudioDeviceIds's scope
    //  * `*_len`       are in/out, properly initialized.
    let code = unsafe { XInputGetAudioDeviceIds(user_index.into(), render_id.as_mut_ptr(), &mut render_len, capture_id.as_mut_ptr(), &mut capture_len) };
    // a dynamic alloc fallback might be appropriate...? what error is returned? experiment, as it's not documented? D3D's own docs show only 256 byte buffers, surely 16x that (4096) is enough?
    check_success!(code)?;
    let render_device_id    = OsString::from_wide(render_id .get(..render_len  as usize).ok_or(fn_param_error!(render_device_id,  ERROR::BUFFER_TOO_SMALL))?.split(|c| *c==0).next().unwrap_or(&[]));
    let capture_device_id   = OsString::from_wide(capture_id.get(..capture_len as usize).ok_or(fn_param_error!(capture_device_id, ERROR::BUFFER_TOO_SMALL))?.split(|c| *c==0).next().unwrap_or(&[]));
    Ok(AudioDeviceIds {
        render_device_id:   if render_device_id .is_empty() { None } else { Some(render_device_id ) },
        capture_device_id:  if capture_device_id.is_empty() { None } else { Some(capture_device_id) },
    })
}

#[test] fn test_returns() {
    if get_audio_device_ids(User::Zero) == ERROR::INVALID_FUNCTION { return }

    // May or may not succeed, even if gamepad not connected
    if let Err(err) = get_audio_device_ids(User::Zero ) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err); }
    if let Err(err) = get_audio_device_ids(User::One  ) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err); }
    if let Err(err) = get_audio_device_ids(User::Two  ) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err); }
    if let Err(err) = get_audio_device_ids(User::Three) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err); }

    // Invalid User s
    assert_eq!(ERROR::BAD_ARGUMENTS, get_audio_device_ids(User::Any));
    for u in User::iter_invalid() {
        assert_eq!(ERROR::BAD_ARGUMENTS, get_audio_device_ids(u));
    }
}
