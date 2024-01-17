use bytemuck::*;

use winapi::shared::winerror::*;

use winresult::{ErrorCode, HResult, HResultError};

use std::fmt::{self, Debug, Display, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/desktop/direct3d11/d3d11-graphics-reference-returnvalues)\]
/// HRESULT
///
/// See [xinput::errors](crate::errors) for a list of constants
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ErrorKind(HRESULT);

enumish! { ErrorKind => HRESULT }

impl ErrorKind {
    #[allow(missing_docs)] pub const fn to_code    (self) -> Option<ErrorCode> { if (self.0 as u32) < 0x10000 { Some(ErrorCode::from_constant(self.0 as _)) } else { None } }
    #[allow(missing_docs)] pub const fn from_winapi(value: HRESULT) -> Self { Self(value) } // TODO: remove
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0 as u32;
        if value < 0x10000 {
            write!(f, "{:?} ({value})", ErrorCode::from(value as u16))
        } else {
            write!(f, "{:?} (0x{value:08X})", HResult::from(value))
        }
    }
}

impl Debug for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.0 as u32;
        if value < 0x10000 {
            write!(f, "{:?} ({value})", ErrorCode::from(value as u16))
        } else {
            write!(f, "{:?} (0x{value:08X})", HResult::from(value))
        }
    }
}

impl std::error::Error for ErrorKind {}

impl From<ErrorCode     > for ErrorKind { fn from(ec: ErrorCode     ) -> Self { Self(u32::from(ec) as _) } }
impl From<HResultError  > for ErrorKind { fn from(hr: HResultError  ) -> Self { Self(u32::from(hr) as _) } }
//impl From<HResult     > for ErrorKind { fn from(hr: HResult       ) -> Self { Self(u32::from(hr) as _) } }

impl PartialEq<HRESULT> for ErrorKind { fn eq(&self, other: &HRESULT)   -> bool { self.0 == *other } }
impl PartialEq<ErrorKind> for HRESULT { fn eq(&self, other: &ErrorKind)    -> bool { other.0 == *self } }

impl PartialEq<Option<ErrorKind>> for ErrorKind { fn eq(&self, other: &Option<ErrorKind>) -> bool { Some(self) == other.as_ref() } }
impl PartialEq<ErrorKind> for Option<ErrorKind> { fn eq(&self, other: &ErrorKind)         -> bool { Some(other) == self.as_ref() } }

impl<O> PartialEq<Result<O, ErrorKind>> for ErrorKind { fn eq(&self, other: &Result<O, ErrorKind>) -> bool { Some(self) == other.as_ref().err() } }
impl<O> PartialEq<ErrorKind> for Result<O, ErrorKind> { fn eq(&self, other: &ErrorKind)            -> bool { Some(other) == self.as_ref().err() } }

impl PartialEq<ErrorCode> for ErrorKind    { fn eq(&self, other: &ErrorCode     ) -> bool { ErrorKind::from(*self) == ErrorKind::from(*other) } }
impl PartialEq<ErrorKind> for ErrorCode    { fn eq(&self, other: &ErrorKind     ) -> bool { ErrorKind::from(*self) == ErrorKind::from(*other) } }
impl PartialEq<HResultError> for ErrorKind { fn eq(&self, other: &HResultError  ) -> bool { ErrorKind::from(*self) == ErrorKind::from(*other) } }
impl PartialEq<ErrorKind> for HResultError { fn eq(&self, other: &ErrorKind     ) -> bool { ErrorKind::from(*self) == ErrorKind::from(*other) } }
