use crate::*;

use bytemuck::Zeroable;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation)\]
/// XInputGetBatteryInformation
/// <span style="opacity: 50%">(1.3+)</span>
///
/// ### Arguments
/// *   `user_index`    Identify which user's controller to get the battery information of
/// *   `dev_type`      [BatteryDevType::Gamepad] or [BatteryDevType::Headset]
///
/// ### Example
/// ```rust
/// use xinput::BatteryDevType;
/// let info = xinput::get_battery_information(0, BatteryDevType::Gamepad).unwrap_or_default();
/// println!("{info:#?}");
/// ```
///
/// ```text
/// BatteryInformation {
///     battery_type: BatteryType::Disconnected,
///     battery_level: BatteryLevel::Empty,
/// }
/// ```
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [error::DEVICE_NOT_CONNECTED]   - Disconnected [`User`]
/// *   [error::DEVICE_NOT_CONNECTED]   - Invalid [`BatteryDevType`]
/// *   [error::INVALID_FUNCTION]       - API unavailable: requires XInput 1.3 or later
pub fn get_battery_information(user_index: impl TryInto<u32>, dev_type: impl Into<BatteryDevType>) -> Result<BatteryInformation, Error> {
    fn_context!(xinput::get_battery_information => XInputGetBatteryInformation);
    #[allow(non_snake_case)] let XInputGetBatteryInformation = Imports::get().XInputGetBatteryInformation;
    let user_index = user_index.try_into().map_err(|_| fn_param_error!(user_index, error::BAD_ARGUMENTS))?;

    let mut info = BatteryInformation::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `user_index`  is well tested
    //  * `dev_type`    is decently tested (0, 1, 2 (OOB), 42, 255 all result in defined behavior)
    //  * `info`        is out-only, no cbSize field, fixed size, sane
    let code = unsafe { XInputGetBatteryInformation(user_index, dev_type.into().into(), info.as_mut()) };
    check_success!(code)?;
    Ok(info)
}

#[test] fn test_valid_params() {
    if let Err(err) = get_battery_information(0, BatteryDevType::Gamepad) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(1, BatteryDevType::Gamepad) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(2, BatteryDevType::Gamepad) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(3, BatteryDevType::Gamepad) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }

    if let Err(err) = get_battery_information(0, BatteryDevType::Headset) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(1, BatteryDevType::Headset) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(2, BatteryDevType::Headset) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(3, BatteryDevType::Headset) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    assert_eq!(error::DEVICE_NOT_CONNECTED, get_battery_information(User::Zero, BatteryDevType::from_unchecked(42))); // bad devtype
    assert_eq!(error::BAD_ARGUMENTS,        get_battery_information(User::Any,  BatteryDevType::Gamepad));
    for u in User::iter_invalid() {
        assert_eq!(error::BAD_ARGUMENTS, get_battery_information(u, BatteryDevType::Gamepad));
    }
}
