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
/// ### Arguments
/// *   `user_index`    &mdash; The controller to get headset/microphone ids for (<code>0 .. [xuser::MAX_COUNT]</code>.)
///
/// ### Example
/// ```rust
/// let audio = xinput::get_audio_device_ids(0).unwrap_or_default();
/// println!("{audio:#?}");
/// ```
///
/// ### Output
/// ```text
/// AudioDeviceIds {
///     render_device_id: None,
///     capture_device_id: None,
/// }
/// ```
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid `user_index` (expected <code>0 .. [xuser::MAX_COUNT]</code>)
/// *   [error::BUFFER_TOO_SMALL]       - Audio device paths exceedingly large (doesn't fit in e.g. `[wchar_t; 4096]`.)
/// *   [error::DEVICE_NOT_CONNECTED]   - **Unreliably.**
/// *   [error::INVALID_FUNCTION]       - API unavailable: requires XInput 1.4 or later
///
/// | System            | Windows `ver`     | Windows SKU           | Behavior |
/// | ----------------- | ----------------- | --------------------- | -------- |
/// | Github Actions    | 10.0.17763.2366   | Windows 2019 Server   | [error::DEVICE_NOT_CONNECTED] observed.
/// | "SACRILEGE"       | 10.0.19041.1415   | Windows 10 Pro        | Succeeds when called on missing gamepads
/// | "NECROMANCY"      | 10.0.19045.3930   | Windows 10 Pro        | [error::DEVICE_NOT_CONNECTED] on a valid XB1 gamepad <br> connected via XB1 wireless dongle <br> (would USB work better? XB360 controllers?)
///
/// ### See Also
/// *   [Getting Audio Device Identifiers](https://learn.microsoft.com/en-us/windows/win32/xinput/getting-started-with-xinput#getting-audio-device-identifiers)
pub fn get_audio_device_ids(user_index: impl TryInto<u32>) -> Result<AudioDeviceIds, Error> {
    fn_context!(xinput::get_audio_device_ids => XInputGetAudioDeviceIds);
    #[allow(non_snake_case)] let XInputGetAudioDeviceIds = imports::XInputGetAudioDeviceIds.load(core::sync::atomic::Ordering::Relaxed);
    let user_index = user_index.try_into().map_err(|_| fn_param_error!(user_index, error::BAD_ARGUMENTS))?;

    let mut render_id  = [0u16; 4096];
    let mut capture_id = [0u16; 4096];
    let mut render_len  = 4096;
    let mut capture_len = 4096;

    // SAFETY: ⚠️ Needs testing with real audio devices
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `user_index`  is well tested
    //  * `*_ptr`       is never null, should only be accessed during XInputGetAudioDeviceIds's scope
    //  * `*_len`       are in/out, properly initialized.
    let code = unsafe { XInputGetAudioDeviceIds(user_index, render_id.as_mut_ptr(), &mut render_len, capture_id.as_mut_ptr(), &mut capture_len) };
    // a dynamic alloc fallback might be appropriate...? what error is returned? experiment, as it's not documented? XInput's own docs show only 256 byte buffers, surely 16x that (4096) is enough?
    check_success!(code)?;
    let render_device_id    = OsString::from_wide(render_id .get(..render_len  as usize).ok_or(fn_param_error!(render_device_id,  error::BUFFER_TOO_SMALL))?.split(|c| *c==0).next().unwrap_or(&[]));
    let capture_device_id   = OsString::from_wide(capture_id.get(..capture_len as usize).ok_or(fn_param_error!(capture_device_id, error::BUFFER_TOO_SMALL))?.split(|c| *c==0).next().unwrap_or(&[]));
    Ok(AudioDeviceIds {
        render_device_id:   if render_device_id .is_empty() { None } else { Some(render_device_id ) },
        capture_device_id:  if capture_device_id.is_empty() { None } else { Some(capture_device_id) },
    })
}


#[test] fn test_valid_args() {
    for user_index in 0 .. 4 {
        if let Err(err) = get_audio_device_ids(user_index) {
            assert!(matches!(err.kind(), error::DEVICE_NOT_CONNECTED | error::INVALID_FUNCTION | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
        }
    }
}

#[test] fn test_bad_user_index() {
    for user_index in xuser::invalids().chain(Some(xuser::INDEX_ANY)) {
        let err = get_audio_device_ids(user_index).expect_err("get_audio_devices_ids should return an error for invalid users");
        assert!(matches!(err.kind(), error::BAD_ARGUMENTS | error::INVALID_FUNCTION | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
    }
}
