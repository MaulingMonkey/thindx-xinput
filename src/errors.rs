//! [ERROR]
//! and [THINERR]
//! [ErrorKind] values<br>
//! **NOTE:** Imported into crate root, no need to prefix `errors::`...
//!
//! | [HRESULT]    | Facility           | Desc  |
//! | ------------:| ------------------ | ----- |
//! | `0x...0....` | FACILITY_NULL      | For broadly applicable common status codes such as S_OK.
//! | `0xA7D8....` |                    | ThinDX Error Codes (A=2\|8=Customer\|Error Bits, 7D8 = TDX = ThinDX)
//!
//! ### See Also
//! *   <https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes#system-error-codes>

#![allow(overflowing_literals)] // ErrorKind is signed for some reason
#![allow(non_snake_case)]
// TODO: Cleanup formatting etc.

#[allow(unused_imports)] use crate::*;
use winresult::HResultError;



/// `0xA7D8....` • **T**hin**DX** [ErrorKind]s
///
/// *   `0xA.......`  - **S**everity and **C**ustomer bits for [HRESULT](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)s
/// *   `0x.7D8....`  - **T**hin **DX** error codes
pub mod THINERR {
    use super::*;

    // General errors

    /// `0xA7D80005`    This version of the DLL doesn't support this fn
    pub const MISSING_DLL_EXPORT : HResultError = HResultError::from_constant(0xA7D80005);

    /// `0xA7D80006`    Slice length exceeded some kind of length limit (typically a conversion to a 32-bit length, or
    ///                 an extra cap introduced by thindx to avoid undefined behavior from allocation size overflows.)
    pub const SLICE_TOO_LARGE : HResultError = HResultError::from_constant(0xA7D80006);
}

/// `0x0000....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-)\] • Non-hresult [ErrorKind](crate::ErrorKind)s
pub use winresult::ERROR;
