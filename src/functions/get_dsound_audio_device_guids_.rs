use crate::*;

use bytemuck::Zeroable;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetdsoundaudiodeviceguids)\]
/// XInputGetDSoundAudioDeviceGuids
/// <span style="opacity: 50%">(..= 1.3)</span>
///
/// Get DirectSound Audio Device GUIDs (N/A for Windows Store apps, isn't supported by Windows 8.)
///
/// ### Arguments
/// *   `user_index`    &mdash; The controller to get headset/microphone ids for (<code>0 .. [xuser::MAX_COUNT]</code>.)
///
/// ### Example
/// ```rust
/// let audio = xinput::get_dsound_audio_device_guids(0).unwrap_or_default();
/// println!("{audio:#?}");
/// ```
///
/// ### Output
/// ```text
/// DSoundAudioDeviceGuids {
///     dsound_render_guid: {00000000-0000-0000-0000-000000000000},
///     dsound_capture_guid: {00000000-0000-0000-0000-000000000000},
/// }
/// ```
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid `user_index` (expected <code>0 .. [xuser::MAX_COUNT]</code>)
/// *   [error::DEVICE_NOT_CONNECTED]   - No gamepad connected for `user_index`
/// *   [error::INVALID_FUNCTION]       - API unavailable: requires XInput 1.3 or earlier
/// *   <span style="opacity: 50%">None</span>  - No audio device(s) connected to gamepad.
#[deprecated = "Deprecated in favor of xinput::get_audio_device_ids.  Unavailable for Windows Store apps, may fail on Windows 8."]
pub fn get_dsound_audio_device_guids(user_index: impl TryInto<u32>) -> Result<DSoundAudioDeviceGuids, Error> {
    fn_context!(xinput::get_dsound_audio_device_guids => XInputGetDSoundAudioDeviceGuids);
    #[allow(non_snake_case)] let XInputGetDSoundAudioDeviceGuids = imports::XInputGetDSoundAudioDeviceGuids.load(core::sync::atomic::Ordering::Relaxed);
    let user_index = user_index.try_into().map_err(|_| fn_param_error!(user_index, error::BAD_ARGUMENTS))?;

    let mut guids = DSoundAudioDeviceGuids::zeroed();
    // SAFETY: ❌ Untested (need a system actually defining XInputGetDSoundAudioDeviceGuids)
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `user_index`  ❌ should be well tested
    //  * `*_guid`      are nice and fixed-size etc.
    let code = unsafe { XInputGetDSoundAudioDeviceGuids(user_index, guids.dsound_render_guid.as_mut(), guids.dsound_capture_guid.as_mut()) };
    check_success!(code)?;
    Ok(guids)
}



#[test] #[allow(deprecated)] fn test_valid_args() {
    for user_index in 0 .. 4 {
        if let Err(err) = get_dsound_audio_device_guids(user_index) {
            assert!(matches!(err.kind(), error::DEVICE_NOT_CONNECTED | error::INVALID_FUNCTION | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
        }
    }
}

#[test] #[allow(deprecated)] fn test_bad_user_index() {
    for user_index in xuser::invalids().chain(Some(xuser::INDEX_ANY)) {
        let err = get_dsound_audio_device_guids(user_index).expect_err("get_dsound_audio_device_guids should return an error for invalid users");
        assert!(matches!(err.kind(), error::BAD_ARGUMENTS | error::INVALID_FUNCTION | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
    }
}
