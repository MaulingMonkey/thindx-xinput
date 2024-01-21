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
/// *   [error::BAD_ARGUMENTS]?         - Invalid `user_index` (expected <code>0 .. [xuser::MAX_COUNT]</code>)?
/// *   [error::DEVICE_NOT_CONNECTED]?  - No gamepad connected for `user_index`?
/// *   [error::INVALID_FUNCTION]       - API unavailable: requires XInput 1.3 or earlier
#[deprecated = "Deprecated in favor of xinput::get_audio_device_ids.  Unavailable for Windows Store apps, may fail on Windows 8."]
pub fn get_dsound_audio_device_guids(user_index: impl TryInto<u32>) -> Result<DSoundAudioDeviceGuids, Error> {
    fn_context!(xinput::get_dsound_audio_device_guids => XInputGetDSoundAudioDeviceGuids);
    #[allow(non_snake_case)] let XInputGetDSoundAudioDeviceGuids = Imports::get().XInputGetDSoundAudioDeviceGuids;
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

#[test] fn test() {
    #[allow(deprecated)] let r = get_dsound_audio_device_guids(0);
    if r != error::INVALID_FUNCTION {
        mmrbi::warning!(at: file!(), line: line!() as usize,
            "xinput::get_dsound_audio_device_guids(0) returned {:?}: may be implemented on this platform: add test coverage!",
            r
        );
    }
}
