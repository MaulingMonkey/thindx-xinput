use crate::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputenable)\]
/// XInputEnable
/// <span style="opacity: 50%">(1.1+)</span>
///
/// Meant to be called when an application gains or loses activation (<code>[WM_ACTIVATEAPP]</code>),
/// to enable or disable XInput for this app.
///
/// "Disabling" xinput for the current process with <code>[xinput](crate)::[enable]\(false\)</code> will:
/// *   Stop all vibration (including if you call <code>[xinput](crate)::[set_state]</code> again.)
/// *   Cause <code>[xinput](crate)::[get_state]</code> to retrieve neutral data for connected controllers (no buttons held, 0ed axises.)
///
/// While this is plenty sufficient for games wanting to handle OS-level application focus,
/// this is not particularly useful if you instead want to enable/disable xinput access on a
/// *per window* or *per focused control* basis instead of application-wide.
/// Wrapping [`get_state`], [`set_state`], etc. to filter these at a more granular level is left
/// as an exercise to the API consumer and whatever wrappers around xinput they might write.
///
/// ### Arguments
/// *   `enable`        &mdash; `true` to accept input and allow vibration, `false` to block input and vibration.
///
/// ### Example
/// ```rust
/// # use winapi::shared::minwindef::*;
/// # use winapi::shared::windef::*;
/// # use winapi::um::winuser::*;
/// unsafe extern "system"
/// fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
///     match msg {
///         WM_ACTIVATEAPP  => { let _ = xinput::enable(wparam as BOOL != FALSE); },
///         _               => {},
///     }
///     DefWindowProcW(hwnd, msg, wparam, lparam)
/// }
/// ```
///
/// ### Errors
/// *   [error::INVALID_FUNCTION]       - API unavailable: requires XInput 1.1 or later
///
/// ### See Also
/// *   <code>[WM_ACTIVATEAPP]</code>   &mdash; Application gains/loses activation.
/// *   <code>[WM_ACTIVATE]</code>      &mdash; Individual top-level window gains/loses activation.  I generally don't recommend calling [`enable`] from this.
/// *   [How is the Focus working in a windows program?](https://web.archive.org/web/20090218140801/http://www.codeguru.com/forum/archive/index.php/t-422817.html)
///     &mdash; [archive.org](https://archive.org/)'ed codeguru.com thread from 2007 with multiple replies by "SuperKoko" detailing message order, the nuances of activated window vs focused window, etc.
///
/// [WM_ACTIVATEAPP]:   https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp
/// [WM_ACTIVATE]:      https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-activate
pub fn enable(enable: bool) -> Result<(), Error> {
    fn_context!(xinput::enable => XInputEnable);
    #[allow(non_snake_case)] let XInputEnable = imports::XInputEnable.load(core::sync::atomic::Ordering::Relaxed);
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `enable`      can be true or false.  Pretty easy to have exhaustive test coverage.
    unsafe { XInputEnable(enable.into()) };
    Ok(())
}

#[test] fn spam_xinput_enable() {
    for e in [true, true, true, false, false, false, true, false, true, false, true, false, true] {
        if let Err(err) = enable(e) {
            assert!(matches!(err.kind(), error::INVALID_FUNCTION | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
        }
    }
}
