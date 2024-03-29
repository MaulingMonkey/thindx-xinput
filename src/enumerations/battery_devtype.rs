#[cfg(doc)] use crate::*;
use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation)\]
/// BATTERY_DEVTYPE_\*
///
/// ### See Also
/// *   [`get_battery_information`]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct BatteryDevType(u8);

enumish! { BatteryDevType => u8; default: Gamepad == 0; Gamepad, Headset }

#[allow(non_upper_case_globals)] impl BatteryDevType {
    /// Get the battery information for a gamepad
    pub const Gamepad : BatteryDevType = BatteryDevType(BATTERY_DEVTYPE_GAMEPAD); // 0

    /// Get the battery information for a headset
    pub const Headset : BatteryDevType = BatteryDevType(BATTERY_DEVTYPE_HEADSET);
}

//#cpp2rust BATTERY_DEVTYPE_GAMEPAD = xinput::BatteryDevType::Gamepad
//#cpp2rust BATTERY_DEVTYPE_HEADSET = xinput::BatteryDevType::Headset
