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
    #[allow(non_snake_case)] let XInputSetState = imports::XInputSetState.load(core::sync::atomic::Ordering::Relaxed);
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
    for user_index in 0 .. 4 {
        if let Err(err) = set_state(user_index, v) {
            assert!(matches!(err.kind(), error::DEVICE_NOT_CONNECTED | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
        }
    }
}

#[test] fn test_bad_user_index() {
    let v = Vibration::default();
    for user_index in xuser::invalids().chain(Some(xuser::INDEX_ANY)) {
        let err = set_state(user_index, v).expect_err("expected error for invalid user_index");
        assert!(matches!(err.kind(), error::BAD_ARGUMENTS | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
    }
}
