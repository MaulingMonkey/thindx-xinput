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
    for user_index in 0..4 {
        for dev_type in [BatteryDevType::Gamepad, BatteryDevType::Headset] {
            if let Err(err) = get_battery_information(user_index, dev_type) {
                assert!(matches!(err.kind(), error::DEVICE_NOT_CONNECTED | error::INVALID_FUNCTION | error::CO_E_NOTINITIALIZED));
            }
        }
    }
}

#[test] fn test_bad_battery_type() {
    // bad BatteryDevType is ignored?
    for dev_type in [3, 42, 250].map(BatteryDevType::from_unchecked) {
        if let Err(err) = get_battery_information(0, dev_type) {
            assert!(matches!(err.kind(), error::DEVICE_NOT_CONNECTED | error::INVALID_FUNCTION | error::CO_E_NOTINITIALIZED), "{err:?}");
        }
    }
}

#[test] fn test_bad_user_index() {
    for user_index in Some(xuser::INDEX_ANY).into_iter().chain(xuser::invalids()) {
        let err = get_battery_information(user_index, BatteryDevType::Gamepad).expect_err("expected error for invalid user_index");
        assert!(matches!(err.kind(), error::BAD_ARGUMENTS | error::INVALID_FUNCTION | error::CO_E_NOTINITIALIZED), "unexpected error type: {err:?}");
    }
}
