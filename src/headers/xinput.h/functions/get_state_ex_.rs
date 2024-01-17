use crate::*;
use bytemuck::Zeroable;



/// \[<strike>microsoft.com</strike>\]
/// XInputGetStateEx
/// <span style="opacity: 50%">(1.3 ..= 1.4)</span>
///
/// ⚠️ **NOTE** ⚠️ This undocumented function is reserved for system software to access [Buttons::Guide].
///
/// Silently falls back on [`XInputGetState`](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate) if `XInputGetStateEx` is unavailable.
///
/// ### Errors
/// *   [error::INVALID_FUNCTION]       - Couldn't find an XInput DLL
/// *   [error::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [error::DEVICE_NOT_CONNECTED]   - [`User`] gamepad not connected
#[deprecated = "This undocumented function is reserved for system software to access Buttons::Guide."]
pub fn get_state_ex(user_index: impl Into<u32>) -> Result<State, Error> {
    fn_context!(xinput::get_state_ex => XInputGetStateEx);
    #[allow(non_snake_case)] let XInputGetStateEx = Imports::get()._XInputGetStateEx;
    let mut state = State::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * tested        in `examples/xinput-exercise-all.rs` (Guide button works)
    //  * `user_index`  is well tested
    //  * `state`       is out-only, fixed size, no `cbSize` field, never null, all bit patterns sane
    //  * `fn`          should be `None` or valid if returned by `Imports::get()`
    let code = unsafe { XInputGetStateEx(user_index.into(), state.as_mut()) };
    check_success!(code)?;
    Ok(state)
}

#[test] #[allow(deprecated)] fn test_valid_params() {
    if let Err(err) = get_state_ex(0u32) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_state_ex(1u32) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_state_ex(2u32) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_state_ex(3u32) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
}

#[test] #[allow(deprecated)] fn test_bad_arguments() {
    assert_eq!(error::BAD_ARGUMENTS, get_state_ex(User::Any));
    for u in User::iter_invalid() {
        assert_eq!(error::BAD_ARGUMENTS, get_state_ex(u));
    }
}
