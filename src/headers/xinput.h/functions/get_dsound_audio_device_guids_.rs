use crate::*;

use bytemuck::Zeroable;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetdsoundaudiodeviceguids)\]
/// XInputGetDSoundAudioDeviceGuids
/// <span style="opacity: 50%">(..= 1.3)</span>
///
/// Get DirectSound Audio Device GUIDs (N/A for Windows Store apps, isn't supported by Windows 8.)
///
/// ### Errors
/// *   [ERROR::INVALID_FUNCTION]       - DirectSound GUIDs unavailable: XInput 1.3 or earlier
/// *   [ERROR::BAD_ARGUMENTS]?         - [`User`] out of bounds?
/// *   [ERROR::DEVICE_NOT_CONNECTED]?  - [`User`] in bounds, but without a gamepad?
#[deprecated = "Deprecated in favor of xinput::get_audio_device_ids.  Unavailable for Windows Store apps, may fail on Windows 8."]
pub fn get_dsound_audio_device_guids(user_index: impl Into<User>) -> Result<DSoundAudioDeviceGuids, Error> {
    fn_context!(xinput::get_dsound_audio_device_guids => XInputGetDSoundAudioDeviceGuids);

    #[allow(non_snake_case)] let XInputGetDSoundAudioDeviceGuids = Imports::get().XInputGetDSoundAudioDeviceGuids;

    let mut guids = DSoundAudioDeviceGuids::zeroed();
    // SAFETY: ❌ Untested (need a system actually defining XInputGetDSoundAudioDeviceGuids)
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `user_index`  ❌ should be well tested
    //  * `*_guid`      are nice and fixed-size etc.
    let code = unsafe { XInputGetDSoundAudioDeviceGuids(user_index.into().into(), guids.dsound_render_guid.as_mut(), guids.dsound_capture_guid.as_mut()) };
    check_success!(code)?;
    Ok(guids)
}

#[test] fn test() {
    #[allow(deprecated)] let r = get_dsound_audio_device_guids(User::Zero);
    if r != ERROR::INVALID_FUNCTION {
        mmrbi::warning!(at: file!(), line: line!() as usize,
            "xinput::get_dsound_audio_device_guids(0) returned {:?}: may be implemented on this platform: add test coverage!",
            r
        );
    }
}
