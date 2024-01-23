use crate::*;

use bytemuck::Zeroable;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate)\]
/// XInputGetState
///
/// Retrieves the current state of the specified controller.
///
/// ### Arguments
/// *   `user_index`    &mdash; The controller to get the state of (<code>0 .. [xuser::MAX_COUNT]</code>.)
///
/// ### Example
/// ```rust
/// let gamepad = 0;
/// let state : xinput::State = xinput::get_state(gamepad).unwrap_or_default();
/// println!("{state:#?}");
/// ```
///
/// ### Output
/// ```text
/// State {
///     packet_number: 305,
///     gamepad: Gamepad {
///         buttons: Buttons::None,
///         left_trigger: 0,
///         right_trigger: 0,
///         left_thumb_x: 2479,
///         left_thumb_y: -707,
///         right_thumb_x: -48,
///         right_thumb_y: -1028,
///     },
/// }
/// ```
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid `user_index` (expected <code>0 .. [xuser::MAX_COUNT]</code>)
/// *   [error::DEVICE_NOT_CONNECTED]   - No gamepad connected for `user_index`.
/// *   [error::INVALID_FUNCTION]       - API unavailable: XInput not loaded
pub fn get_state(user_index: impl TryInto<u32>) -> Result<State, Error> {
    fn_context!(xinput::get_state => XInputGetState);
    #[allow(non_snake_case)] let XInputGetState = imports::XInputGetState.load(core::sync::atomic::Ordering::Relaxed);
    let user_index = user_index.try_into().map_err(|_| fn_param_error!(user_index, error::BAD_ARGUMENTS))?;

    let mut state = State::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * tested        in `examples/d3d9-02-xinput.rs`
    //  * `user_index`  is well tested
    //  * `state`       is out-only, fixed size, no `cbSize` field, never null, all bit patterns sane
    let code = unsafe { XInputGetState(user_index, state.as_mut()) };
    check_success!(code)?;
    Ok(state)
}

#[test] fn test_valid_params() {
    for user_index in 0 .. 4 {
        if let Err(err) = get_state(user_index) {
            assert!(matches!(err.kind(), error::DEVICE_NOT_CONNECTED | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
        }
    }
}

#[test] fn test_bad_user_index() {
    for user_index in xuser::invalids().chain(Some(xuser::INDEX_ANY)) {
        let err = get_state(user_index).expect_err("expected error for invalid user_index");
        assert!(matches!(err.kind(), error::BAD_ARGUMENTS | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
    }
}
