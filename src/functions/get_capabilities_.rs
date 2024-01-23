use crate::*;

use bytemuck::Zeroable;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities)\]
/// XInputGetCapabilities
///
/// ### Arguments
/// *   `user_index`    &mdash; The controller to get capabilities and features for (<code>0 .. [xuser::MAX_COUNT]</code>.)
/// *   `flags`         &mdash; [`Flag::None`] or [`Flag::Gamepad`].
///
/// ### Example
/// ```rust
/// let caps = xinput::get_capabilities(0, xinput::Flag::None);
/// println!("{caps:#?}");
/// ```
///
/// ### Output
/// ```text
/// Ok(
///     Capabilities {
///         ty: DevType::Gamepad,
///         sub_type: DevSubType::Gamepad,
///         flags: Caps::None,
///         gamepad: Gamepad {
///             buttons: Buttons::{DPadUp|DPadDown|DPadLeft|DPadRight|Start|Back|LeftThumb|RightThumb|LeftShoulder|RightShoulder|A|B|X|Y},
///             left_trigger: 255,
///             right_trigger: 255,
///             left_thumb_x: -64,
///             left_thumb_y: -64,
///             right_thumb_x: -64,
///             right_thumb_y: -64,
///         },
///         vibration: Vibration {
///             left_motor_speed: 255,
///             right_motor_speed: 255,
///         },
///     },
/// )
/// ```
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid [`Flag`]
/// *   [error::BAD_ARGUMENTS]          - Invalid `user_index` (expected <code>0 .. [xuser::MAX_COUNT]</code>)
/// *   [error::DEVICE_NOT_CONNECTED]   - [`Flag::None`]
/// *   [error::DEVICE_NOT_CONNECTED]   - No gamepad connected for `user_index`.
/// *   [error::INVALID_FUNCTION]       - API unavailable: XInput not loaded
pub fn get_capabilities(user_index: impl TryInto<u32>, flags: Flag) -> Result<Capabilities, Error> {
    fn_context!(xinput::get_capabilities => XInputGetCapabilities);
    #[allow(non_snake_case)] let XInputGetCapabilities = imports::XInputGetCapabilities.load(core::sync::atomic::Ordering::Relaxed);
    let user_index = user_index.try_into().map_err(|_| fn_param_error!(user_index, error::BAD_ARGUMENTS))?;

    let mut caps = Capabilities::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `user_index`  is well tested
    //  * `flags`       is decently tested (0, 1, 2 (OOB), 4, 8, 16, 32, 64, 128, 0xFFFFFFFF)
    //  * `caps`        is out-only, no cbSize field, fixed size, sane
    let code = unsafe { XInputGetCapabilities(user_index, flags.into(), caps.as_mut()) };
    check_success!(code)?;
    Ok(caps)
}

#[test] fn test_valid_params() {
    for user_index in 0 .. 4 {
        for flag in [Flag::None, Flag::Gamepad] {
            if let Err(err) = get_capabilities(user_index, flag) {
                assert!(matches!(err.kind(), error::DEVICE_NOT_CONNECTED | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
            }
        }
    }
}

#[test] fn test_bad_user_index() {
    for user_index in xuser::invalids().chain(Some(xuser::INDEX_ANY)) {
        for flag in [Flag::None, Flag::Gamepad, Flag::from_unchecked(42), Flag::from_unchecked(!0)] {
            let err = get_capabilities(user_index, flag).expect_err("get_capabilities should return an error on a bad user_index");
            assert!(matches!(err.kind(), error::BAD_ARGUMENTS | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
        }
    }
}

#[test] fn test_bad_flags() {
    for user_index in 0 .. 4 {
        for flag in [Flag::from_unchecked(42), Flag::from_unchecked(!0)] {
            let err = get_capabilities(user_index, flag).expect_err("get_capabilities should return an error on a bad flag");
            assert!(matches!(err.kind(), error::BAD_ARGUMENTS | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
        }
    }
}
