use crate::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputsetstate)\]
/// XInputSetState
///
/// Control the vibration of a controller.
///
/// ### Arguments
/// *   `user_index`    &mdash; The controller to vibrate (<code>0 .. [xuser::MAX_COUNT]</code>.)
/// *   `vibration`     &mdash; How much [`Vibration`] the controller's motors should provide.
///
/// ### Example
/// ```rust
/// // No rumble
/// let gamepad = 0;
/// let _ = xinput::set_state(gamepad, [0, 0]);
/// ```
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid `user_index` (expected <code>0 .. [xuser::MAX_COUNT]</code>)
/// *   [error::DEVICE_NOT_CONNECTED]   - No gamepad connected for `user_index`.
/// *   [error::INVALID_FUNCTION]       - API unavailable: XInput not loaded
///
/// ### Recommendations
/// Game engine developers should strongly consider wrapping this function in a higher level system or API that:
/// 1.  Exposes an API for fire-and-forget rumble events (with e.g. falloff curves.)
///     *   Avoids the need for each caller to implement tedious animation/falloff logic.
///     *   Avoids the common bug of permanent rumble when per-frame callbacks are unregistered before rumble reaches 0.
/// 2.  Implements any manual control over longer term rumble events by e.g. "replacing" or updating rumble event state.
///     *   The loss of e.g. per-frame updates to a long term rumble event should "fail safe" by (eventually?) falling off to 0 rumble.
///     *   Anything else is incredibly bug-prone in my experience.
/// 3.  Allows summing/mixing rumble from multiple sources simultaniously.
///     *   Avoids the problem of a subtle rumble effect silencing a loud one thanks to in-frame call order.
///
/// Such an API might also consider:
/// *   Abstracting away the fact that it uses xinput under the hood.
/// *   Implementing position-based events, such that explosions generate rumble per-controller by distance from said explosion.
/// *   Being (primarily) an implementation detail of a more abstract "game events" API.
///     Gameplay code might simply trigger an "explosive_barrel_explosion" event, which in turn triggers audio, particles, rumble, lighting, achievement progress, etc.
///     These in turn could be parameterized by some designer-configurable table somewhere with "explosive_barrel_explosion" as the key, requiring no programmer intervention to add/change/remove rumble.
///
/// ### Common Issues
/// This function *non-exclusively* controls/mutates *global* hardware state.
/// This inevitably means bugs when other software attempts to control the same state at the same time.
/// If your attempts to rumble the controller don't seem to be working right:
///
/// *   Try closing other games.  It's common for them to set vibration on a per-frame basis.  When no vibration is necessary, this may mean they'll set vibration to 0 *every frame*.  Those games should've used [`enable`], but perhaps they didn't.
/// *   Try closing game editors and related tools.  It's common for them to reuse code from their associated games, which might mean they're also attempting to 0 vibration every frame.  Those editors/tools should've used [`enable`], but perhaps they didn't.
/// *   You might have multiple instances of the same game running (especially if testing multiplayer!)  Consider using [`enable`] appropriately such that only the focused one gets input / controls rumble.
/// *   Try setting a breakpoint by function name for `XInputSetState`:
///     *   A coworker may have introduced a new system that thinks it should control vibration by setting it every frame.
///     *   An old system may exist that already controls vibration by setting it every frame.
///     *   Third party middleware such as [WWise's Motion plugin](https://www.youtube.com/watch?v=I-2aR7McfKw) may think it should control vibration via your sound editor.
pub fn set_state(user_index: impl TryInto<u32>, vibration: impl Into<Vibration>) -> Result<(), Error> {
    fn_context!(xinput::set_state => XInputSetState);
    #[allow(non_snake_case)] let XInputSetState = imports::XInputSetState.load(core::sync::atomic::Ordering::Relaxed);
    let user_index = user_index.try_into().map_err(|_| fn_param_error!(user_index, error::BAD_ARGUMENTS))?;
    let mut vibration = vibration.into();

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
