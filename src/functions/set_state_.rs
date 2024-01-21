use crate::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputsetstate)\]
/// XInputSetState
///
/// Control the vibration of a controller.
///
/// ### Arguments
/// *   `user_index`    &mdash; The controller to vibrate (<code>0 .. [xuser::MAX_COUNT]</code>.)
///
/// ### Example
/// ```rust
/// // No rumble
/// let gamepad = 0;
/// let _ = xinput::set_state(gamepad, xinput::Vibration {
///     left_motor_speed:   0u16,
///     right_motor_speed:  0u16,
/// });
/// ```
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid `user_index` (expected <code>0 .. [xuser::MAX_COUNT]</code>)
/// *   [error::DEVICE_NOT_CONNECTED]   - No gamepad connected for `user_index`.
/// *   [error::INVALID_FUNCTION]       - API unavailable: XInput not loaded
pub fn set_state(user_index: impl TryInto<u32>, mut vibration: Vibration) -> Result<(), Error> {
    fn_context!(xinput::set_state => XInputSetState);
    #[allow(non_snake_case)] let XInputSetState = Imports::get().XInputSetState;
    let user_index = user_index.try_into().map_err(|_| fn_param_error!(user_index, error::BAD_ARGUMENTS))?;

    // SAFETY: ✔️
    //  * fuzzed        in `fuzz-xinput.rs`
    //  * tested        in `d3d9-02-xinput.rs`
    //  * `user_index`  is well tested
    //  * `vibration`   is never null, fixed size, no `cbSize` field, all bit patterns are valid and reasonable
    let code = unsafe { XInputSetState(user_index, vibration.as_mut()) };
    check_success!(code)
}

#[test] fn test_valid_params() {
    let v = Vibration::default();
    if let Err(err) = set_state(0, v) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = set_state(1, v) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = set_state(2, v) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = set_state(3, v) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    let v = Vibration::default();
    assert_eq!(error::BAD_ARGUMENTS, set_state(xuser::INDEX_ANY, v));
    for u in xuser::invalids() {
        assert_eq!(error::BAD_ARGUMENTS, set_state(u, v));
    }
}
