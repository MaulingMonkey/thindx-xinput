use crate::*;

use bytemuck::Zeroable;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate)\]
/// XInputGetState
///
/// Retrieves the current state of the specified controller.
///
/// ### Example
/// ```rust
/// let gamepad : u32 = 0;
/// let state : xinput::State = xinput::get_state(gamepad).unwrap_or_default();
/// println!("{state:#?}");
/// ```
///
/// ```text
/// State {
///     packet_number: 0,
///     gamepad: Gamepad {
///         buttons: Buttons::None,
///         left_trigger: 0,
///         right_trigger: 0,
///         left_thumb_x: 0,
///         left_thumb_y: 0,
///         right_thumb_x: 0,
///         right_thumb_y: 0,
///     },
/// }
/// ```
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [error::DEVICE_NOT_CONNECTED]   - [`User`] gamepad not connected
/// *   [error::INVALID_FUNCTION]       - API unavailable: XInput not loaded
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
    if let Err(err) = get_state(0u32) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_state(1u32) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_state(2u32) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_state(3u32) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    assert_eq!(error::BAD_ARGUMENTS, get_state(User::Any));
    for u in User::iter_invalid() {
        assert_eq!(error::BAD_ARGUMENTS, get_state(u));
    }
}
