#![doc = include_str!("../Readme.md")]
//!
//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-game-controller-apis-portal)\]
//! APIs for Xbox 360 style controllers
//!
//! ### Enumerations
//! | C++                       | Rust                  |
//! | ------------------------- | --------------------- |
//! | [`BATTERY_DEVTYPE_*`]     | <code>[BatteryDevType]::\*</code>
//! | [`BATTERY_LEVEL_*`]       | <code>[BatteryLevel]::\*</code>
//! | [`BATTERY_TYPE_*`]        | <code>[BatteryType]::\*</code>
//! | [`XINPUT_DEVSUBTYPE_*`]   | <code>[DevSubType]::\*</code>
//! | [`XINPUT_DEVTYPE_*`]      | <code>[DevType]::\*</code>
//! | `XUSER_*`                 | <code>[User]::\*</code>
//! | [`VK_*`]                  | <code>[VK]::\*</code>
//!
//! [`BATTERY_DEVTYPE_*`]:      https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation
//! [`BATTERY_LEVEL_*`]:        https://learn.microsoft.com/en-us/windows/win32/api/XInput/ns-xinput-xinput_battery_information
//! [`BATTERY_TYPE_*`]:         https://learn.microsoft.com/en-us/windows/win32/api/XInput/ns-xinput-xinput_battery_information
//! [`XINPUT_DEVSUBTYPE_*`]:    https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-and-controller-subtypes
//! [`XINPUT_DEVTYPE_*`]:       https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_capabilities
//! [`VK_*`]:                   https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_keystroke#remarks
//!
//!
//!
//! ### Flags
//! | C++                       | Rust                  |
//! | ------------------------- | --------------------- |
//! | [`XINPUT_GAMEPAD_*`]      | <code>[`Buttons`]::*</code>
//! | [`XINPUT_CAPS_*`]         | <code>[`Caps`]::*</code>
//! | [`XINPUT_FLAG_*`]         | <code>[`Flag`]::*</code>
//!
//! [`XINPUT_GAMEPAD_*`]:       https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#members
//! [`XINPUT_CAPS_*`]:          https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_capabilities#members
//! [`XINPUT_FLAG_*`]:          https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities#parameters
//!
//!
//!
//! ### Functions
//! | C++                                   | Rust                  |
//! | ------------------------------------- | --------------------- |
//! | [`XInputEnable`]                      | [`enable`]
//! | [`XInputGetAudioDeviceIds`]           | [`get_audio_device_ids`]
//! | [`XInputGetBatteryInformation`]       | [`get_battery_information`]
//! | [`XInputGetCapabilities`]             | [`get_capabilities`]
//! | [`XInputGetDSoundAudioDeviceGuids`] ❌ | [`get_dsound_audio_device_guids`]
//! | [`XInputGetKeystroke`]                | [`get_keystroke`]
//! | [`XInputGetState`]                    | [`get_state`]
//! | `XInputGetStateEx` ⚠️ 1.3             | [`get_state_ex`]
//! | [`XInputSetState`]                    | [`set_state`]
//! | `XInputWaitForGuideButton`    ⚠️ 1.3  | <span style="opacity: 25%">TODO?</span>
//! | `XInputCancelGuideButtonWait` ⚠️ 1.3  | <span style="opacity: 25%">TODO?</span>
//! | `XInputPowerOffController`    ⚠️ 1.3  | <span style="opacity: 25%">TODO?</span>
//! | `XInputGetBaseBusInformation` ⚠️ 1.4  | <span style="opacity: 25%">TODO?</span>
//! | `XInputGetCapabilitiesEx`     ⚠️ 1.4  | <span style="opacity: 25%">TODO?</span>
//!
//! | Legend | Desc |
//! | ------ | ---- |
//! | ❌     | Deprecated
//! | ⚠️ 1.3 | Undocumented XInput 1.3+ function exported by ordinal only
//! | ⚠️ 1.4 | Undocumented XInput 1.4+ function exported by ordinal only
//!
//! [`XInputEnable`]:                       https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputenable
//! [`XInputGetAudioDeviceIds`]:            https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetaudiodeviceids
//! [`XInputGetBatteryInformation`]:        https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation
//! [`XInputGetCapabilities`]:              https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities
//! [`XInputGetDSoundAudioDeviceGuids`]:    https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetdsoundaudiodeviceguids
//! [`XInputGetKeystroke`]:                 https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetkeystroke
//! [`XInputGetState`]:                     https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate
//! [`XInputSetState`]:                     https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputsetstate
//!
//!
//!
//! ### Structures
//! | C++                               | Rust                  |
//! | --------------------------------- | --------------------- |
//! | [`XINPUT_BATTERY_INFORMATION`]    | [`BatteryInformation`]
//! | [`XINPUT_CAPABILITIES`]           | [`Capabilities`]
//! | [`XINPUT_GAMEPAD`]                | [`Gamepad`]
//! | [`XINPUT_KEYSTROKE`]              | [`Keystroke`]
//! | [`XINPUT_STATE`]                  | [`State`]
//! | [`XINPUT_VIBRATION`]              | [`Vibration`]
//! | <code>std::array&lt;[std::wstring], 2&gt;</code>  | [`AudioDeviceIds`]
//! | <code>std::array&lt;[GUID], 2&gt;</code>          | [`DSoundAudioDeviceGuids`]
//!
//! [`XINPUT_BATTERY_INFORMATION`]:     https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_battery_information
//! [`XINPUT_CAPABILITIES`]:            https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_capabilities
//! [`XINPUT_GAMEPAD`]:                 https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad
//! [`XINPUT_KEYSTROKE`]:               https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_keystroke
//! [`XINPUT_STATE`]:                   https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_state
//! [`XINPUT_VIBRATION`]:               https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_vibration
//! [std::wstring]:                     https://learn.microsoft.com/en-us/cpp/standard-library/string-typedefs?view=msvc-170#wstring
//! [GUID]:                             https://learn.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid
//!
//!
//!
//! # Alternatives
//!
//! \[[microsoft.com](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee416842(v=vs.85))\] **DirectInput**
//! *   ✔️ Supports joysticks with many more buttons and axises than XInput.
//! *   ✔️ Leverages Windows's built in support for configuring idle positions, deadzones.
//! *   ⚠️ Older, "deprecated" in favor of XInput.
//! *   ❌ Xbox 360 controllers map both triggers to a single axis in DirectInput.
//! *   [Comparison of XInput and DirectInput features](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-and-directinput)
//!     discusses how to use a hybrid approach of XInput for 360 controllers and DirectInput for non-XInput devices
//!
//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/uwp/gaming/input-for-games)\] **UWP**
//! *   ✔️ Supports Xbox One trigger rumble (XInput only supports base controller rumble)
//! *   ⚠️ Can't recieve input through UWP when the app/window is not active (useful for dev cruft.)
//! *   ⚠️ No Windows 7 support?

#![debugger_visualizer(natvis_file = "../guid.natvis")]
#![debugger_visualizer(natvis_file = "../xinput.natvis")]

#![deny(unsafe_op_in_unsafe_fn)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![deny(unreachable_patterns)] // probably improperly `match { ... }`ed constants

#![allow(clippy::identity_op)]                  // I like to `<< 0`, `+ 0`, etc. for consistency
#![allow(clippy::missing_safety_doc)]           // I prefer ⚠️ Safety ⚠️ docs
#![allow(clippy::derivable_impls)]              // I do this a lot for explicitness with d3d enums
#![allow(clippy::too_many_arguments)]           // 1:1 mapping to D3D... I don't have much of a choice!

// #![warn(clippy::undocumented_unsafe_blocks)]    // too noisy to implement yet

#[doc(no_inline)] pub use ::winresult::ERROR;
#[doc(no_inline)] pub use ::winresult::ErrorHResultOrCode as ErrorKind;



#[macro_use] mod macros;
#[macro_use] mod error_macros;

mod error;                                                      pub use error::*;
#[path="headers/guiddef.h/guiddef.rs"]          mod guiddef_h;  pub use guiddef_h::*;
#[path="headers/xinput.h/xinput.rs"]            mod xinput_h;   pub use xinput_h::*;

//#[cfg(doc)] #[doc = include_str!("../doc/changelog.md")] pub mod _changelog {}
#[cfg(doc)] pub mod _headers;
