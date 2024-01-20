use crate::*;



/// \[<strike>microsoft.com</strike>\]
/// XInputPowerOffController
/// <span style="opacity: 50%">(1.3 ..= 1.4)</span>
///
/// Power off battery powered controllers.
///
/// ⚠️ **NOTE** ⚠️ This undocumented function is reserved for system software.
///
/// ### Arguments
/// *   `user_index`    The controller to power off
///
/// ### Example
/// ```rust
/// # return;
/// #[allow(deprecated)] let _ = xinput::power_off_controller(0);
/// ```
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [error::DEVICE_NOT_CONNECTED]   - [`User`] is not connected
/// *   [error::DEVICE_NOT_CONNECTED]   - XB1 controller connected through XB1 wireless dongle cannot be turned off.
/// *   [error::INVALID_FUNCTION]       - API unavailable: requires XInput 1.3 or 1.4
#[deprecated = "This undocumented function is reserved for system software."]
pub fn power_off_controller(user_index: impl TryInto<u32>) -> Result<(), Error> {
    fn_context!(xinput::power_off_controller => XInputPowerOffController);
    #[allow(non_snake_case)] let XInputPowerOffController = Imports::get()._XInputPowerOffController;
    let user_index = user_index.try_into().map_err(|_| fn_param_error!(user_index, error::BAD_ARGUMENTS))?;

    let code = unsafe { XInputPowerOffController(user_index) };
    check_success!(code)
}

#[test] #[allow(deprecated)] fn test_valid_params() {
    if power_off_controller(128) == error::INVALID_FUNCTION { return }

    //if let Err(err) = power_off_controller(0) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); } // leave on
    if let Err(err) = power_off_controller(1) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = power_off_controller(2) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = power_off_controller(3) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
}

#[test] #[allow(deprecated)] fn test_bad_arguments() {
    if power_off_controller(128) == error::INVALID_FUNCTION { return }

    assert_eq!(error::BAD_ARGUMENTS, power_off_controller(User::Any));
    for u in User::iter_invalid() {
        assert_eq!(error::BAD_ARGUMENTS, power_off_controller(u));
    }
}
