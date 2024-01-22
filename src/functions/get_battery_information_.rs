use crate::*;

use bytemuck::Zeroable;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation)\]
/// XInputGetBatteryInformation
/// <span style="opacity: 50%">(1.3+)</span>
///
/// ### Arguments
/// *   `user_index`    &mdash; The controller to get battery status for (<code>0 .. [xuser::MAX_COUNT]</code>.)
/// *   `dev_type`      &mdash; [BatteryDevType::Gamepad] or [BatteryDevType::Headset].
///
/// ### Example
/// ```rust
/// use xinput::BatteryDevType;
/// for battery_ty in [xinput::BatteryDevType::Gamepad, xinput::BatteryDevType::Headset] {
///     let info = xinput::get_battery_information(0, battery_ty).unwrap_or_default();
///     println!("{battery_ty:?} = {info:#?}");
/// }
/// ```
///
/// ### Output
/// ```text
/// BatteryDevType::Gamepad = BatteryInformation {
///     battery_type: BatteryType::Alkaline,
///     battery_level: BatteryLevel::Full,
/// }
/// BatteryDevType::Headset = BatteryInformation {
///     battery_type: BatteryType::Alkaline,
///     battery_level: BatteryLevel::Full,
/// }
/// ```
///
/// N.B. `Headset` battery was reported, even with no headset plugged into my XB1 controller (connected via Xbox One USB bluetooth dongle.)
/// Xbox 360 controllers may behave differently?
///
/// ### Errors
/// *   [error::BAD_ARGUMENTS]          - Invalid `user_index` (expected <code>0 .. [xuser::MAX_COUNT]</code>)
/// *   [error::DEVICE_NOT_CONNECTED]   - No gamepad connected for `user_index`.
/// *   [error::DEVICE_NOT_CONNECTED]   - Invalid [`BatteryDevType`] ?  Sometimes?
/// *   [error::INVALID_FUNCTION]       - API unavailable: requires XInput 1.3 or later
pub fn get_battery_information(user_index: impl TryInto<u32>, dev_type: impl Into<BatteryDevType>) -> Result<BatteryInformation, Error> {
    fn_context!(xinput::get_battery_information => XInputGetBatteryInformation);
    #[allow(non_snake_case)] let XInputGetBatteryInformation = imports::XInputGetBatteryInformation.load(core::sync::atomic::Ordering::Relaxed);
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
    // bad BatteryDevType is ignored?
    if let Err(err) = get_battery_information(0, BatteryDevType::from_unchecked(  3)) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(0, BatteryDevType::from_unchecked( 42)) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(0, BatteryDevType::from_unchecked(250)) { assert_eq!(err, error::DEVICE_NOT_CONNECTED); }

    assert_eq!(error::DEVICE_NOT_CONNECTED, get_battery_information(3, BatteryDevType::from_unchecked(42))); // disconnected gamepad takes precedence
    assert_eq!(error::BAD_ARGUMENTS,        get_battery_information(xuser::INDEX_ANY, BatteryDevType::Gamepad));
    for u in xuser::invalids() {
        assert_eq!(error::BAD_ARGUMENTS, get_battery_information(u, BatteryDevType::Gamepad));
    }
}
