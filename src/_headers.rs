// This file is autogenerated by _xtask.rs

#![warn(rustdoc::broken_intra_doc_links)]

//! Rust ⮀ C++ coverage information based on Windows SDK 10.0.22621.0
//!
//! ⚠️ Scanned C++ definitions are not yet complete.
//! Based on [MaulingMonkey/windows-sdk-scanner](https://github.com/MaulingMonkey/windows-sdk-scanner).
//!
//! # Headers
//!
//! | C++ Header | Interfaces | Structs | Enums | Functions | Constants | Macros |
//! | ---------- | ---------- | ------- | ----- | --------- | --------- | ------ |
//! | [guiddef.h](const@guiddef_h) |   | ✔️ 1 of 1 |   |   | ⚠️ 1 of 7 | ⚠️ 4 of 7 |
//! | [xinput.h](const@xinput_h) |   | ✔️ 6 of 6 |   | ✔️ 8 of 8 | ⚠️ 83 of 86 |   |

use crate as xinput;



/// # guiddef.h
///
/// ### C++ Structs -> Rust Structs
///
/// [`GUID`](https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid)&nbsp;→ [`Guid`] <br>
/// ### C++ Constants → Rust Constants
///
/// `CLSID_NULL`&nbsp;→&nbsp;❌ <br>
/// `FMTID_NULL`&nbsp;→&nbsp;❌ <br>
/// `IID_NULL`&nbsp;→&nbsp;❌ <br>
/// `REFCLSID`&nbsp;→&nbsp;❌ <br>
/// `REFFMTID`&nbsp;→&nbsp;❌ <br>
/// `REFGUID`&nbsp;→ <code>&[Guid](Guid)</code> <br>
/// `REFIID`&nbsp;→&nbsp;❌ <br>
/// ### C++ Macros → Rust fns/macros
///
/// `DEFINE_GUID`&nbsp;→ [`guid!`] <br>
/// `DEFINE_OLEGUID`&nbsp;→&nbsp;❌ <br>
/// `InlineIsEqualGUID`&nbsp;→ [`Guid::eq`] <br>
/// `IsEqualCLSID`&nbsp;→&nbsp;❌ <br>
/// `IsEqualFMTID`&nbsp;→&nbsp;❌ <br>
/// `IsEqualGUID`&nbsp;→ [`Guid::eq`] <br>
/// `IsEqualIID`&nbsp;→&nbsp;❌ <br>
pub const guiddef_h : cxx_header = cxx_header;



/// # xinput.h
///
/// ### C++ Structs -> Rust Structs
///
/// [`XINPUT_BATTERY_INFORMATION`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_battery_information)&nbsp;→ [`xinput::BatteryInformation`] <br>
/// [`XINPUT_CAPABILITIES`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_capabilities)&nbsp;→ [`xinput::Capabilities`] <br>
/// [`XINPUT_GAMEPAD`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad)&nbsp;→ [`xinput::Gamepad`] <br>
/// [`XINPUT_KEYSTROKE`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_keystroke)&nbsp;→ [`xinput::Keystroke`] <br>
/// [`XINPUT_STATE`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_state)&nbsp;→ [`xinput::State`] <br>
/// [`XINPUT_VIBRATION`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_vibration)&nbsp;→ [`xinput::Vibration`] <br>
/// ### C++ Functions → Rust Fns
///
/// [`XInputEnable`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputenable)&nbsp;→ [`xinput::enable`] <br>
/// [`XInputGetAudioDeviceIds`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetaudiodeviceids)&nbsp;→ [`xinput::get_audio_device_ids`] <br>
/// [`XInputGetBatteryInformation`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation)&nbsp;→ [`xinput::get_battery_information`] <br>
/// [`XInputGetCapabilities`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities)&nbsp;→ [`xinput::get_capabilities`] <br>
/// [`XInputGetDSoundAudioDeviceGuids`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetdsoundaudiodeviceguids)&nbsp;→ [`xinput::get_dsound_audio_device_guids`] <br>
/// [`XInputGetKeystroke`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetkeystroke)&nbsp;→ [`xinput::get_keystroke`] <br>
/// [`XInputGetState`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate)&nbsp;→ [`xinput::get_state`] <br>
/// [`XInputSetState`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputsetstate)&nbsp;→ [`xinput::set_state`] <br>
/// ### C++ Constants → Rust Constants
///
/// `BATTERY_DEVTYPE_GAMEPAD`&nbsp;→ [`xinput::BatteryDevType::Gamepad`] <br>
/// `BATTERY_DEVTYPE_HEADSET`&nbsp;→ [`xinput::BatteryDevType::Headset`] <br>
/// `BATTERY_LEVEL_EMPTY`&nbsp;→ [`xinput::BatteryLevel::Empty`] <br>
/// `BATTERY_LEVEL_FULL`&nbsp;→ [`xinput::BatteryLevel::Full`] <br>
/// `BATTERY_LEVEL_LOW`&nbsp;→ [`xinput::BatteryLevel::Low`] <br>
/// `BATTERY_LEVEL_MEDIUM`&nbsp;→ [`xinput::BatteryLevel::Medium`] <br>
/// `BATTERY_TYPE_ALKALINE`&nbsp;→ [`xinput::BatteryType::Alkaline`] <br>
/// `BATTERY_TYPE_DISCONNECTED`&nbsp;→ [`xinput::BatteryType::Disconnected`] <br>
/// `BATTERY_TYPE_NIMH`&nbsp;→ [`xinput::BatteryType::NiMH`] <br>
/// `BATTERY_TYPE_UNKNOWN`&nbsp;→ [`xinput::BatteryType::Unknown`] <br>
/// `BATTERY_TYPE_WIRED`&nbsp;→ [`xinput::BatteryType::Wired`] <br>
/// `VK_PAD_A`&nbsp;→ [`xinput::VK::PadA`] <br>
/// `VK_PAD_B`&nbsp;→ [`xinput::VK::PadB`] <br>
/// `VK_PAD_BACK`&nbsp;→ [`xinput::VK::PadBack`] <br>
/// `VK_PAD_DPAD_DOWN`&nbsp;→ [`xinput::VK::PadDPadDown`] <br>
/// `VK_PAD_DPAD_LEFT`&nbsp;→ [`xinput::VK::PadDPadLeft`] <br>
/// `VK_PAD_DPAD_RIGHT`&nbsp;→ [`xinput::VK::PadDPadRight`] <br>
/// `VK_PAD_DPAD_UP`&nbsp;→ [`xinput::VK::PadDPadUp`] <br>
/// `VK_PAD_LSHOULDER`&nbsp;→ [`xinput::VK::PadLShoulder`] <br>
/// `VK_PAD_LTHUMB_DOWN`&nbsp;→ [`xinput::VK::PadLThumbDown`] <br>
/// `VK_PAD_LTHUMB_DOWNLEFT`&nbsp;→ [`xinput::VK::PadLThumbDownLeft`] <br>
/// `VK_PAD_LTHUMB_DOWNRIGHT`&nbsp;→ [`xinput::VK::PadLThumbDownRight`] <br>
/// `VK_PAD_LTHUMB_LEFT`&nbsp;→ [`xinput::VK::PadLThumbLeft`] <br>
/// `VK_PAD_LTHUMB_PRESS`&nbsp;→ [`xinput::VK::PadLThumbPress`] <br>
/// `VK_PAD_LTHUMB_RIGHT`&nbsp;→ [`xinput::VK::PadLThumbRight`] <br>
/// `VK_PAD_LTHUMB_UP`&nbsp;→ [`xinput::VK::PadLThumbUp`] <br>
/// `VK_PAD_LTHUMB_UPLEFT`&nbsp;→ [`xinput::VK::PadLThumbUpLeft`] <br>
/// `VK_PAD_LTHUMB_UPRIGHT`&nbsp;→ [`xinput::VK::PadLThumbUpRight`] <br>
/// `VK_PAD_LTRIGGER`&nbsp;→ [`xinput::VK::PadLTrigger`] <br>
/// `VK_PAD_RSHOULDER`&nbsp;→ [`xinput::VK::PadRShoulder`] <br>
/// `VK_PAD_RTHUMB_DOWN`&nbsp;→ [`xinput::VK::PadRThumbDown`] <br>
/// `VK_PAD_RTHUMB_DOWNLEFT`&nbsp;→ [`xinput::VK::PadRThumbDownLeft`] <br>
/// `VK_PAD_RTHUMB_DOWNRIGHT`&nbsp;→ [`xinput::VK::PadRThumbDownRight`] <br>
/// `VK_PAD_RTHUMB_LEFT`&nbsp;→ [`xinput::VK::PadRThumbLeft`] <br>
/// `VK_PAD_RTHUMB_PRESS`&nbsp;→ [`xinput::VK::PadRThumbPress`] <br>
/// `VK_PAD_RTHUMB_RIGHT`&nbsp;→ [`xinput::VK::PadRThumbRight`] <br>
/// `VK_PAD_RTHUMB_UP`&nbsp;→ [`xinput::VK::PadRThumbUp`] <br>
/// `VK_PAD_RTHUMB_UPLEFT`&nbsp;→ [`xinput::VK::PadRThumbUpLeft`] <br>
/// `VK_PAD_RTHUMB_UPRIGHT`&nbsp;→ [`xinput::VK::PadRThumbUpRight`] <br>
/// `VK_PAD_RTRIGGER`&nbsp;→ [`xinput::VK::PadRTrigger`] <br>
/// `VK_PAD_START`&nbsp;→ [`xinput::VK::PadStart`] <br>
/// `VK_PAD_X`&nbsp;→ [`xinput::VK::PadX`] <br>
/// `VK_PAD_Y`&nbsp;→ [`xinput::VK::PadY`] <br>
/// `XINPUT_CAPS_FFB_SUPPORTED`&nbsp;→ [`xinput::Caps::FfbSupported`] <br>
/// `XINPUT_CAPS_NO_NAVIGATION`&nbsp;→ [`xinput::Caps::NoNavigation`] <br>
/// `XINPUT_CAPS_PMD_SUPPORTED`&nbsp;→ [`xinput::Caps::PmdSupported`] <br>
/// `XINPUT_CAPS_VOICE_SUPPORTED`&nbsp;→ [`xinput::Caps::VoiceSupported`] <br>
/// `XINPUT_CAPS_WIRELESS`&nbsp;→ [`xinput::Caps::Wireless`] <br>
/// `XINPUT_DEVSUBTYPE_ARCADE_PAD`&nbsp;→ [`xinput::DevSubType::ArcadePad`] <br>
/// `XINPUT_DEVSUBTYPE_ARCADE_STICK`&nbsp;→ [`xinput::DevSubType::ArcadeStick`] <br>
/// `XINPUT_DEVSUBTYPE_DANCE_PAD`&nbsp;→ [`xinput::DevSubType::DancePad`] <br>
/// `XINPUT_DEVSUBTYPE_DRUM_KIT`&nbsp;→ [`xinput::DevSubType::DrumKit`] <br>
/// `XINPUT_DEVSUBTYPE_FLIGHT_STICK`&nbsp;→ [`xinput::DevSubType::FlightStick`] <br>
/// `XINPUT_DEVSUBTYPE_GAMEPAD`&nbsp;→ [`xinput::DevSubType::Gamepad`] <br>
/// `XINPUT_DEVSUBTYPE_GUITAR`&nbsp;→ [`xinput::DevSubType::Guitar`] <br>
/// `XINPUT_DEVSUBTYPE_GUITAR_ALTERNATE`&nbsp;→ [`xinput::DevSubType::GuitarAlternate`] <br>
/// `XINPUT_DEVSUBTYPE_GUITAR_BASS`&nbsp;→ [`xinput::DevSubType::GuitarBass`] <br>
/// `XINPUT_DEVSUBTYPE_UNKNOWN`&nbsp;→ [`xinput::DevSubType::Unknown`] <br>
/// `XINPUT_DEVSUBTYPE_WHEEL`&nbsp;→ [`xinput::DevSubType::Wheel`] <br>
/// `XINPUT_DEVTYPE_GAMEPAD`&nbsp;→ [`xinput::DevType::Gamepad`] <br>
/// `XINPUT_DLL` →&nbsp;❌ <br>
/// `XINPUT_DLL_A` →&nbsp;❌ <br>
/// `XINPUT_DLL_W` →&nbsp;❌ <br>
/// `XINPUT_FLAG_GAMEPAD`&nbsp;→ [`xinput::Flag::Gamepad`] <br>
/// `XINPUT_GAMEPAD_A`&nbsp;→ [`xinput::Buttons::A`] <br>
/// `XINPUT_GAMEPAD_B`&nbsp;→ [`xinput::Buttons::B`] <br>
/// `XINPUT_GAMEPAD_BACK`&nbsp;→ [`xinput::Buttons::Back`] <br>
/// `XINPUT_GAMEPAD_DPAD_DOWN`&nbsp;→ [`xinput::Buttons::DPadDown`] <br>
/// `XINPUT_GAMEPAD_DPAD_LEFT`&nbsp;→ [`xinput::Buttons::DPadLeft`] <br>
/// `XINPUT_GAMEPAD_DPAD_RIGHT`&nbsp;→ [`xinput::Buttons::DPadRight`] <br>
/// `XINPUT_GAMEPAD_DPAD_UP`&nbsp;→ [`xinput::Buttons::DPadUp`] <br>
/// `XINPUT_GAMEPAD_LEFT_SHOULDER`&nbsp;→ [`xinput::Buttons::LeftShoulder`] <br>
/// `XINPUT_GAMEPAD_LEFT_THUMB`&nbsp;→ [`xinput::Buttons::LeftThumb`] <br>
/// `XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE`&nbsp;→ [`xinput::Gamepad::LEFT_THUMB_DEADZONE`] <br>
/// `XINPUT_GAMEPAD_RIGHT_SHOULDER`&nbsp;→ [`xinput::Buttons::RightShoulder`] <br>
/// `XINPUT_GAMEPAD_RIGHT_THUMB`&nbsp;→ [`xinput::Buttons::RightThumb`] <br>
/// `XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE`&nbsp;→ [`xinput::Gamepad::RIGHT_THUMB_DEADZONE`] <br>
/// `XINPUT_GAMEPAD_START`&nbsp;→ [`xinput::Buttons::Start`] <br>
/// `XINPUT_GAMEPAD_TRIGGER_THRESHOLD`&nbsp;→ [`xinput::Gamepad::TRIGGER_THRESHOLD`] <br>
/// `XINPUT_GAMEPAD_X`&nbsp;→ [`xinput::Buttons::X`] <br>
/// `XINPUT_GAMEPAD_Y`&nbsp;→ [`xinput::Buttons::Y`] <br>
/// `XINPUT_KEYSTROKE_KEYDOWN`&nbsp;→ [`xinput::Keystroke::KeyDown`] <br>
/// `XINPUT_KEYSTROKE_KEYUP`&nbsp;→ [`xinput::Keystroke::KeyUp`] <br>
/// `XINPUT_KEYSTROKE_REPEAT`&nbsp;→ [`xinput::Keystroke::Repeat`] <br>
/// `XUSER_INDEX_ANY`&nbsp;→ [`xinput::User::Any`] <br>
/// `XUSER_MAX_COUNT`&nbsp;→ [`xinput::User::MAX_COUNT`] <br>
pub const xinput_h : cxx_header = cxx_header;



#[allow(non_camel_case_types)] #[doc(hidden)] pub struct cxx_header;
use crate::*;
