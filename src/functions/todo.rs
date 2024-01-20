//! XInput constants/functions that have yet to be implemented, or will not be implemented.
//!
//! The functions are all undocumented.



/// A DLL name to feed into `LoadLibrary` etc. - varies based on Windows SDK / DirectX SDK, as well as ambient defines.
/// Since this crate also handles loading from multiple versions automagically under the hood, this isn't terribly necessary.
pub const DLL   : &'static str = "xinput_???.dll";

/// Ambient narrow system codepage version of [`DLL`].
/// Presumes the system locale is an ASCII variant, which is a bad assumption.
/// [`abistr`](https://docs.rs/abistr/) could help give this a saner type...?
pub const DLL_A : &'static str = "xinput_???.dll";

/// Ambient wide version of [`DLL`].
/// Would require a dependency on [`abistr`](https://docs.rs/abistr/) or similar.
pub const DLL_W : &'static str = "xinput_???.dll";



/// \[<strike>microsoft.com</strike>\]
/// XInputWaitForGuideButton
/// <span style="opacity: 50%">(1.3 ..= 1.4)</span>
pub fn wait_for_guide_button() { todo!() }

/// \[<strike>microsoft.com</strike>\]
/// XInputCancelGuideButtonWait
/// <span style="opacity: 50%">(1.3 ..= 1.4)</span>
pub fn cancel_guide_button_wait() { todo!() }

/// \[<strike>microsoft.com</strike>\]
/// XInputGetBaseBusInformation
/// <span style="opacity: 50%">(1.4 only)</span>
pub fn get_base_bus_information() { todo!() }

/// \[<strike>microsoft.com</strike>\]
/// XInputGetCapabilitiesEx
/// <span style="opacity: 50%">(1.4 only)</span>
pub fn get_capabilities_ex() { todo!() }
