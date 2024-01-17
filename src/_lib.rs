#![doc = include_str!("../Readme.md")]
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

mods! {
    inl mod namespaces {
        inl mod _root;
        inl mod xinput;
    }

    #[path="headers/guiddef.h/guiddef.rs"]          mod guiddef_h;
    #[path="headers/xinput.h/xinput.rs"]            mod xinput_h;

    mod error;
}

//#[cfg(doc)] #[doc = include_str!("../doc/changelog.md")] pub mod _changelog {}
#[cfg(doc)] pub mod _headers;
