use crate::*;

use bytemuck::Zeroable;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities)\]
/// XInputGetCapabilities
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
/// *   [error::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [error::DEVICE_NOT_CONNECTED]   - [`Flag::None`]
/// *   [error::DEVICE_NOT_CONNECTED]   - [`User`] in bounds, but without a gamepad
/// *   [error::INVALID_FUNCTION]       - API unavailable: XInput not loaded
pub fn get_capabilities(user_index: impl TryInto<u32>, flags: Flag) -> Result<Capabilities, Error> {
    fn_context!(xinput::get_capabilities => XInputGetCapabilities);
    #[allow(non_snake_case)] let XInputGetCapabilities = Imports::get().XInputGetCapabilities;
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
    if let Err(err) = get_capabilities(0, Flag::Gamepad) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(1, Flag::Gamepad) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(2, Flag::Gamepad) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(3, Flag::Gamepad) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }

    if let Err(err) = get_capabilities(0, Flag::None   ) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(1, Flag::None   ) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(2, Flag::None   ) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(3, Flag::None   ) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    assert_eq!(error::BAD_ARGUMENTS,        get_capabilities(User::Any,                 Flag::Gamepad));            // Bad User (any)
    assert_eq!(error::BAD_ARGUMENTS,        get_capabilities(User::from_unchecked(4),   Flag::Gamepad));            // Bad User (obb)
    assert_eq!(error::BAD_ARGUMENTS,        get_capabilities(User::Zero,                Flag::from_unchecked(42))); // Bad Flag (obb)
    assert_eq!(error::BAD_ARGUMENTS,        get_capabilities(User::Zero,                Flag::from_unchecked(!0))); // Bad Flag (obb)
    for u in User::iter_invalid() {
        assert_eq!(error::BAD_ARGUMENTS, get_capabilities(u, Flag::Gamepad)); // Bad user only
        assert_eq!(error::BAD_ARGUMENTS, get_capabilities(u, Flag::from_unchecked(42))); // Bad Flag (obb)
        assert_eq!(error::BAD_ARGUMENTS, get_capabilities(u, Flag::from_unchecked(!0))); // Bad Flag (obb)
    }
}
