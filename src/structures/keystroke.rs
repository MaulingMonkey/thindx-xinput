use crate::*;
use bytemuck::{Pod, Zeroable};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_keystroke)\]
/// XINPUT_KEYSTROKE
///
/// Describes a gamepad-provided keystroke.
#[derive(Clone, Copy, Debug)]
#[derive(Default, Pod, Zeroable)]
#[repr(C)] pub struct Keystroke {
    /// Virtual key code of the key/button/stick movement.
    pub virtual_key:    VK,

    /// Documented as being unused?
    pub unicode:        u16,

    /// [`Keystroke::KeyDown`] | [`Keystroke::KeyUp`] | [`Keystroke::Repeat`]
    pub flags:          KeystrokeFlags,

    /// Index of the signed-in gamer associated with the device. Can be a value in the range 0–3.
    pub user_index:     u8,

    /// HID code corresponding to the input. If there is no corresponding HID code, this value is zero.
    pub hid_code:       u8,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Keystroke => winapi::um::xinput::XINPUT_KEYSTROKE {
        virtual_key     => VirtualKey,
        unicode         => Unicode,
        flags           => Flags,
        user_index      => UserIndex,
        hid_code        => HidCode,
    }
}

#[test] fn test_traits_for_coverage() {
    let _keystroke = Keystroke::zeroed();
    let _keystroke = _keystroke.clone();
    dbg!(_keystroke);
}

//#cpp2rust XINPUT_KEYSTROKE            = xinput::Keystroke
