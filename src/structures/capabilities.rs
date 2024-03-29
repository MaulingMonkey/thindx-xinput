use crate::*;
use bytemuck::{Pod, Zeroable};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_capabilities)\]
/// XINPUT_CAPABILITIES
///
/// Battery type and charge.
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct Capabilities {
    /// Device type (generally always [DevType::Gamepad]?)
    pub ty:         DevType,

    /// Device "sub"type.
    ///
    /// **NOTE:** "Legacy" XInput (9.1.0 / Windows Vista) will always return [`DevSubType::Gamepad`], regardless of device.
    pub sub_type:   DevSubType,

    /// Capability flags.
    pub flags:      Caps,

    /// Describes available features and control resolutions.
    pub gamepad:    Gamepad,

    /// Describes available functionality and resolutions.
    pub vibration:  Vibration,
}

impl AsRef<Self> for Capabilities { fn as_ref(&    self) -> &    Self { self } }
impl AsMut<Self> for Capabilities { fn as_mut(&mut self) -> &mut Self { self } }

#[test] fn test_traits_for_coverage() {
    let _caps = Capabilities::zeroed();
    let _caps = _caps.clone();
    dbg!(_caps);
}

//#cpp2rust XINPUT_CAPABILITIES         = xinput::Capabilities
