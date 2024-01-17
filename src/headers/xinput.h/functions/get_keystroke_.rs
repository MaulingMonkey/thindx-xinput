use crate::*;

use bytemuck::Zeroable;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetkeystroke)\]
/// XInputGetKeystroke
///
/// Retrieves gamepad input events.
///
/// ### Errors
/// *   [ERROR::INVALID_FUNCTION]       - Keystrokes API unavailable: requires XInput 1.3+
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - Disconnected [`User`]
/// *   [ERROR::EMPTY]                  → Returns <code>[Ok]\([None]\)</code> instead.
pub fn get_keystroke(user_index: impl Into<u32>, _reserved: ()) -> Result<Option<Keystroke>, Error> {
    fn_context!(xinput::get_keystroke => XInputGetKeystroke);
    #[allow(non_snake_case)] let XInputGetKeystroke = Imports::get().XInputGetKeystroke.ok_or(fn_error!(ERROR::INVALID_FUNCTION))?;
    let mut keystroke = Keystroke::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * tested        in `examples/xinput-exercise-all.rs`
    //  * `user_index`  is well tested
    let code = unsafe { XInputGetKeystroke(user_index.into(), 0, keystroke.as_mut()) };
    if code == ERROR::EMPTY.to_u32() { return Ok(None) }
    check_success!(code)?;
    Ok(Some(keystroke))
}

#[test] fn test_valid_args() {
    if let Err(err) = get_keystroke(User::Zero,  ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err); }
    if let Err(err) = get_keystroke(User::One,   ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err); }
    if let Err(err) = get_keystroke(User::Two,   ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err); }
    if let Err(err) = get_keystroke(User::Three, ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err); }
    if let Err(err) = get_keystroke(User::Any,   ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err); }
}

#[test] fn test_invalid_args() {
    // User::Any is valid
    for u in User::iter_invalid() {
        assert_eq!(ERROR::BAD_ARGUMENTS, get_keystroke(u, ()));
    }
}
