use crate::*;

use bytemuck::Zeroable;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate)\]
/// XInputGetState
///
/// Retrieves the current state of the specified controller.
///
/// ### Errors
/// *   [ERROR::INVALID_FUNCTION]       - Couldn't find an XInput DLL
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - [`User`] gamepad not connected
pub fn get_state(user_index: impl Into<u32>) -> Result<State, Error> {
    fn_context!(xinput::get_state => XInputGetState);
    #[allow(non_snake_case)] let XInputGetState = Imports::get().XInputGetState;
    let mut state = State::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * tested        in `examples/d3d9-02-xinput.rs`
    //  * `user_index`  is well tested
    //  * `state`       is out-only, fixed size, no `cbSize` field, never null, all bit patterns sane
    let code = unsafe { XInputGetState(user_index.into(), state.as_mut()) };
    check_success!(code)?;
    Ok(state)
}

#[test] fn test_valid_params() {
    if let Err(err) = get_state(0u32) { assert_eq!(err, ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_state(1u32) { assert_eq!(err, ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_state(2u32) { assert_eq!(err, ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_state(3u32) { assert_eq!(err, ERROR::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    assert_eq!(ERROR::BAD_ARGUMENTS, get_state(User::Any));
    for u in User::iter_invalid() {
        assert_eq!(ERROR::BAD_ARGUMENTS, get_state(u));
    }
}
