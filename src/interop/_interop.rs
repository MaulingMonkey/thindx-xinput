#[cfg(feature = "winapi-0-3")] #[path = "winapi-0.3.rs"] mod winapi_0_3;
#[cfg(feature = "winapi-0-2")] #[path = "winapi-0.2.rs"] mod winapi_0_2;
#[cfg(feature = "winapi-0-1")] #[path = "winapi-0.1.rs"] mod winapi_0_1;

#[cfg(feature = "windows-0-52")] mod windows_0_52 { use windows_0_52::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.44.rs"); }
#[cfg(feature = "windows-0-51")] mod windows_0_51 { use windows_0_51::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.44.rs"); }
#[cfg(feature = "windows-0-48")] mod windows_0_48 { use windows_0_48::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.44.rs"); }
#[cfg(feature = "windows-0-46")] mod windows_0_46 { use windows_0_46::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.44.rs"); }
#[cfg(feature = "windows-0-44")] mod windows_0_44 { use windows_0_44::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.44.rs"); }
#[cfg(feature = "windows-0-43")] mod windows_0_43 { use windows_0_43::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-42")] mod windows_0_42 { use windows_0_42::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-41")] mod windows_0_41 { use windows_0_41::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-40")] mod windows_0_40 { use windows_0_40::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-39")] mod windows_0_39 { use windows_0_39::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-38")] mod windows_0_38 { use windows_0_38::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-37")] mod windows_0_37 { use windows_0_37::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-36")] mod windows_0_36 { use windows_0_36::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-35")] mod windows_0_35 { use windows_0_35::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-34")] mod windows_0_34 { use windows_0_34::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-33")] mod windows_0_33 { use windows_0_33::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-32")] mod windows_0_32 { use windows_0_32::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-30")] mod windows_0_30 { use windows_0_30::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-29")] mod windows_0_29 { use windows_0_29::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-28")] mod windows_0_28 { use windows_0_28::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-27")] mod windows_0_27 { use windows_0_27::{core::GUID, Win32::UI::Input::XboxController::*};    include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-26")] mod windows_0_26 { use windows_0_26::{runtime::GUID, Win32::UI::Input::XboxController::*}; include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-25")] mod windows_0_25 { use windows_0_25::{runtime::GUID, Win32::UI::Input::XboxController::*}; include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-24")] mod windows_0_24 { use windows_0_24::{runtime::GUID, Win32::UI::Input::XboxController::*}; include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-23")] mod windows_0_23 { use windows_0_23::{runtime::GUID, Win32::UI::XInput::*};                include!("windows-0.22.rs"); }
#[cfg(feature = "windows-0-22")] mod windows_0_22 { use windows_0_22::{runtime::GUID, Win32::UI::XInput::*};                include!("windows-0.22.rs"); }

#[cfg(feature = "windows-sys-0-52")] mod windows_sys_0_52 { use windows_sys_0_52 as windows_sys; include!("windows-sys-0.45.rs"); }
#[cfg(feature = "windows-sys-0-48")] mod windows_sys_0_48 { use windows_sys_0_48 as windows_sys; include!("windows-sys-0.45.rs"); }
#[cfg(feature = "windows-sys-0-45")] mod windows_sys_0_45 { use windows_sys_0_45 as windows_sys; include!("windows-sys-0.45.rs"); }

#[cfg(feature = "windows-sys-0-42")] mod windows_sys_0_42 { use windows_sys_0_42 as windows_sys; include!("windows-sys-0.27.rs"); }
#[cfg(feature = "windows-sys-0-36")] mod windows_sys_0_36 { use windows_sys_0_36 as windows_sys; include!("windows-sys-0.27.rs"); }
#[cfg(feature = "windows-sys-0-35")] mod windows_sys_0_35 { use windows_sys_0_35 as windows_sys; include!("windows-sys-0.27.rs"); }
#[cfg(feature = "windows-sys-0-34")] mod windows_sys_0_34 { use windows_sys_0_34 as windows_sys; include!("windows-sys-0.27.rs"); }
#[cfg(feature = "windows-sys-0-33")] mod windows_sys_0_33 { use windows_sys_0_33 as windows_sys; include!("windows-sys-0.27.rs"); }
#[cfg(feature = "windows-sys-0-32")] mod windows_sys_0_32 { use windows_sys_0_32 as windows_sys; include!("windows-sys-0.27.rs"); }
#[cfg(feature = "windows-sys-0-30")] mod windows_sys_0_30 { use windows_sys_0_30 as windows_sys; include!("windows-sys-0.27.rs"); }
#[cfg(feature = "windows-sys-0-29")] mod windows_sys_0_29 { use windows_sys_0_29 as windows_sys; include!("windows-sys-0.27.rs"); }
#[cfg(feature = "windows-sys-0-28")] mod windows_sys_0_28 { use windows_sys_0_28 as windows_sys; include!("windows-sys-0.27.rs"); }
#[cfg(feature = "windows-sys-0-27")] mod windows_sys_0_27 { use windows_sys_0_27 as windows_sys; include!("windows-sys-0.27.rs"); }
