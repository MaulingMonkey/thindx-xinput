use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_keystroke#members)\]
/// XINPUT_KEYSTROKE_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct KeystrokeFlags(u16);
use KeystrokeFlags as Keystroke;

flags! { Keystroke => u16; None, KeyDown, KeyUp, Repeat }

#[allow(non_upper_case_globals)] impl KeystrokeFlags {
    /// No flags set
    pub const None : Keystroke = Keystroke(0);

    /// The key was pressed.
    pub const KeyDown : Keystroke = Keystroke(XINPUT_KEYSTROKE_KEYDOWN);

    /// The key was released.
    pub const KeyUp : Keystroke = Keystroke(XINPUT_KEYSTROKE_KEYUP);

    /// This was a repeated key event.
    pub const Repeat : Keystroke = Keystroke(XINPUT_KEYSTROKE_REPEAT);
}

#[allow(non_upper_case_globals)] impl crate::Keystroke {
    /// No flags set
    pub const None : Keystroke = Keystroke(0);

    /// The key was pressed.
    pub const KeyDown : Keystroke = Keystroke(XINPUT_KEYSTROKE_KEYDOWN);

    /// The key was released.
    pub const KeyUp : Keystroke = Keystroke(XINPUT_KEYSTROKE_KEYUP);

    /// This was a repeated key event.
    pub const Repeat : Keystroke = Keystroke(XINPUT_KEYSTROKE_REPEAT);
}

//#cpp2rust XINPUT_KEYSTROKE_KEYDOWN    = xinput::Keystroke::KeyDown
//#cpp2rust XINPUT_KEYSTROKE_KEYUP      = xinput::Keystroke::KeyUp
//#cpp2rust XINPUT_KEYSTROKE_REPEAT     = xinput::Keystroke::Repeat
