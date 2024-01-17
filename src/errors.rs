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



/// `0x0000....` • \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-)\] • Non-hresult [ErrorKind](crate::ErrorKind)s
pub use winresult::ERROR;
