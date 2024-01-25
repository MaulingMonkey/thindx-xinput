#![doc = include_str!("../Readme.md")]

#![debugger_visualizer(natvis_file = "../xinput.natvis")]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

#![deny(unsafe_op_in_unsafe_fn)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![deny(unreachable_patterns)] // probably improperly `match { ... }`ed constants

#![allow(clippy::identity_op)]                  // I like to `<< 0`, `+ 0`, etc. for consistency
#![allow(clippy::missing_safety_doc)]           // I prefer ⚠️ Safety ⚠️ docs
#![allow(clippy::derivable_impls)]              // I do this a lot for explicitness with d3d enums
#![allow(clippy::too_many_arguments)]           // 1:1 mapping to XInput... I don't have much of a choice!

// #![warn(clippy::undocumented_unsafe_blocks)]    // too noisy to implement yet

#[cfg(doc)] #[path = "../doc/_doc.rs"] pub mod _doc;
#[macro_use] mod macros;
#[macro_use] mod error_macros;

pub mod error; #[doc(no_inline)] pub use error::Error;

macro_rules! check_success { ( $err:expr ) => { $crate::check_error_success(&_THINDX_FN_CONTEXT, $err) } }

mods! {
    inl mod constants {
        pub mod xuser;
    }

    inl mod dll {
        pub(crate) mod imports;
    }

    inl mod enumerations {
        inl mod battery_devtype;
        inl mod battery_level;
        inl mod battery_type;
        inl mod devsubtype;
        inl mod devtype;
        inl mod vk;
    }

    inl mod flags {
        inl mod buttons;
        inl mod caps;
        inl mod flag;
        inl mod keystroke;
    }

    inl mod functions {
        inl mod enable_;
        inl mod get_audio_device_ids_;
        inl mod get_battery_information_;
        inl mod get_capabilities_;
        inl mod get_dsound_audio_device_guids_;
        inl mod get_keystroke_;
        inl mod get_state_;
        inl mod set_state_;
        #[cfg(doc)] pub mod todo;
    }

    inl mod structures {
        inl mod audio_device_ids;
        inl mod battery_information;
        inl mod capabilities;
        inl mod dsound_audio_device_guids;
        inl mod gamepad;
        inl mod keystroke;
        inl mod state;
        inl mod vibration;
    }
}

include!("functions/get_state_ex_.rs");
include!("functions/power_off_controller_.rs");

pub(crate) fn check_error_success(fn_context: &'static crate::error_macros::FnContext, err: u32) -> Result<(), Error> {
    if err == winresult::ERROR::SUCCESS.to_u32() {
        Ok(())
    } else {
        Err(Error(fn_context, error::Kind::from_u32(err)))
    }
}
