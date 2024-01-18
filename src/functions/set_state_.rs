use crate::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputsetstate)\]
/// XInputSetState
///
/// Control the vibration of a controller.
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [error::DEVICE_NOT_CONNECTED]   - [`User`] is not connected
/// *   [error::INVALID_FUNCTION]       - API unavailable: XInput not loaded
pub fn set_state(user_index: impl Into<u32>, mut vibration: Vibration) -> Result<(), Error> {
    fn_context!(xinput::set_state => XInputSetState);
    #[allow(non_snake_case)] let XInputSetState = Imports::get().XInputSetState;
    // SAFETY: ✔️
    //  * fuzzed        in `fuzz-xinput.rs`
    //  * tested        in `d3d9-02-xinput.rs`
    //  * `user_index`  is well tested
    //  * `vibration`   is never null, fixed size, no `cbSize` field, all bit patterns are valid and reasonable
    let code = unsafe { XInputSetState(user_index.into(), vibration.as_mut()) };
    check_success!(code)
}

#[test] fn test_valid_params() {
    let v = Vibration::default();
    if let Err(err) = set_state(0u32, v) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = set_state(1u32, v) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = set_state(2u32, v) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = set_state(3u32, v) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    let v = Vibration::default();
    assert_eq!(error::BAD_ARGUMENTS, set_state(User::Any, v));
    for u in User::iter_invalid() {
        assert_eq!(error::BAD_ARGUMENTS, set_state(u, v));
    }
}