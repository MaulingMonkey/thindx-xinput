//! [Alternative APIs](apis),
//! [Assumptions](assumptions),
//! [Alternative Crates](crates),
//! [Crate Features](features),
//! [Design Decisions](design),
//! [Environment Variables](environment),
//! [XInput Versions](versions)

use crate::{self as xinput, *, todo::*};

macro_rules! docs {
    ( $($ident:ident),+ $(,)? ) => {$(
        #[doc = include_str!(concat!(stringify!($ident), ".md"))] pub mod $ident {}
    )+};
}

docs! {
    apis,
    assumptions,
    crates,
    design,
    environment,
    features,
    versions,
}
