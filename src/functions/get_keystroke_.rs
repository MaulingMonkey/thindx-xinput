use crate::*;

use bytemuck::Zeroable;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetkeystroke)\]
/// XInputGetKeystroke
/// <span style="opacity: 50%">(1.3+)</span>
///
/// Retrieves gamepad input events.
///
/// ### Argumen.t
/// *   `user_index`    &mdash; The controller to get button/keystrokes for (<code>0 .. [xuser::MAX_COUNT]</code> or [`xuser::INDEX_ANY`].)
/// *   `_reserved`     &mdash; Reserved for future use, simply pass `()`.
///
/// ### Example
/// ```rust
/// // Wait for any gamepad button
/// let gamepad = 0;
/// loop {
///     match xinput::get_keystroke(gamepad, ()) {
///         Ok(Some(keystroke)) => break println!("{keystroke:#?}"),
/// #       Ok(None) if true    => break, // don't actually wait
///         Ok(None)            => std::thread::yield_now(), // wait
///         Err(err)            => break println!("no xinput or no gamepad: {err:?}"),
///     }
/// }
/// ```
///
/// ### Output
/// ```text
/// Keystroke {
///     virtual_key: VK::PadStart,
///     unicode: 0,
///     flags: Keystroke::KeyDown,
///     user_index: 0,
///     hid_code: 0,
/// }
/// ```
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid `user_index` (expected <code>0 .. [xuser::MAX_COUNT]</code> or [`xuser::INDEX_ANY`])
/// *   [error::DEVICE_NOT_CONNECTED]   - No gamepad connected for `user_index`.
/// *   ~~error::EMPTY~~                - No [`Keystroke`]s available.  Returns <code>[Ok]\([None]\)</code> instead.
/// *   [error::INVALID_FUNCTION]       - API unavailable: requires XInput 1.3 or later
pub fn get_keystroke(user_index: impl TryInto<u32>, _reserved: ()) -> Result<Option<Keystroke>, Error> {
    fn_context!(xinput::get_keystroke => XInputGetKeystroke);
    #[allow(non_snake_case)] let XInputGetKeystroke = imports::XInputGetKeystroke.load(core::sync::atomic::Ordering::Relaxed);
    let user_index = user_index.try_into().map_err(|_| fn_param_error!(user_index, error::BAD_ARGUMENTS))?;

    let mut keystroke = Keystroke::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * tested        in `examples/xinput-exercise-all.rs`
    //  * `user_index`  is well tested
    let code = unsafe { XInputGetKeystroke(user_index, 0, keystroke.as_mut()) };
    if code == winresult::ERROR::EMPTY.to_u32() { return Ok(None) }
    check_success!(code)?;
    Ok(Some(keystroke))
}

#[test] fn test_valid_args() {
    if let Err(err) = get_keystroke(0,                ()) { assert_eq!(error::DEVICE_NOT_CONNECTED, err); }
    if let Err(err) = get_keystroke(1,                ()) { assert_eq!(error::DEVICE_NOT_CONNECTED, err); }
    if let Err(err) = get_keystroke(2,                ()) { assert_eq!(error::DEVICE_NOT_CONNECTED, err); }
    if let Err(err) = get_keystroke(3,                ()) { assert_eq!(error::DEVICE_NOT_CONNECTED, err); }
    if let Err(err) = get_keystroke(xuser::INDEX_ANY, ()) { assert_eq!(error::DEVICE_NOT_CONNECTED, err); }
}

#[test] fn test_invalid_args() {
    // xuser::INDEX_ANY is valid
    for u in xuser::invalids() {
        assert_eq!(error::BAD_ARGUMENTS, get_keystroke(u, ()));
    }
}
