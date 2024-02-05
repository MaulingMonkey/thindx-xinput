//! [`Error`], [`Kind`].  Error codes are *rarely* `HRESULT`s (`XInputUap.dll` can return [`CO_E_NOTINITIALIZED`].)<br>
//! [`BAD_ARGUMENTS`], [`BUFFER_TOO_SMALL`], [`DEVICE_NOT_CONNECTED`], [`INVALID_FUNCTION`]

#[cfg(doc)] use crate::*;
use crate::error_macros::FnContext;

use winresult::*;

use core::fmt::{self, Debug, Display, Formatter};



/// An error returned by some XInput function (or wrapper thereof.)
#[derive(Clone)]
pub struct Error(pub(crate) &'static FnContext, pub(crate) Kind);

impl Error {
    pub(crate) fn method(&self) -> &'static str { self.0.directx_method.unwrap_or(self.0.thindx_method) }

    /// Returns the [`Kind`] of the error
    pub fn kind(&self) -> Kind { self.1 }
}

impl Debug   for Error { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "Error({:?}, {:?})", self.method(), self.1) } }
impl Display for Error { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{} failed, returning {:?}", self.method(), self.1) } }

impl std::error::Error for Error {}
impl From<Error> for std::io::Error { fn from(err: Error) -> Self { std::io::Error::new(std::io::ErrorKind::Other, err) } }

impl PartialEq<Error> for ErrorCode { fn eq(&self, other: &Error    ) -> bool { other.kind() == *self } }
impl PartialEq<Error> for Kind      { fn eq(&self, other: &Error    ) -> bool { other.kind() == *self } }
impl PartialEq<ErrorCode> for Error { fn eq(&self, other: &ErrorCode) -> bool { self.kind() == *other } }
impl PartialEq<Kind     > for Error { fn eq(&self, other: &Kind     ) -> bool { self.kind() == *other } }

impl<O, E: PartialEq<Error>> PartialEq<Error> for Result<O, E> { fn eq(&self, other: &Error       ) -> bool { match self.as_ref() { Ok(_) => false, Err(e) => *e == *other } } }
impl<O, E: PartialEq<Error>> PartialEq<Result<O, E>> for Error { fn eq(&self, other: &Result<O, E>) -> bool { match other.as_ref() { Ok(_) => false, Err(e) => *e == *self } } }



/// ≈ <code>[winresult]::[ErrorCode]</code> corresponding to <code>[winresult]::[ERROR]::*</code>, but 32-bit, and oriented around XInput error codes.<br>
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Kind(u32);

impl Kind {
    #[allow(missing_docs)] pub const fn to_u32(self) -> u32 { self.0 }

    pub(crate) const fn new(code: ErrorCode) -> Kind { Kind(code.to_u32()) }
    pub(crate) const fn from_u32(code: u32) -> Self { Self(code) }
    pub(crate) fn to_winresult(self) -> winresult::ErrorHResultOrCode { winresult::ErrorHResultOrCode::from(self.0) }
}

impl Debug for Kind { fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { Debug::fmt(&self.to_winresult(), f) } }

impl PartialEq<Kind> for ErrorCode     { fn eq(&self, other: &Kind         ) -> bool { other.to_winresult() == *self } }
impl PartialEq<Kind> for HResultError  { fn eq(&self, other: &Kind         ) -> bool { other.to_winresult() == *self } }
impl PartialEq<ErrorCode    > for Kind { fn eq(&self, other: &ErrorCode    ) -> bool { self.to_winresult() == *other } }
impl PartialEq<HResultError > for Kind { fn eq(&self, other: &HResultError ) -> bool { self.to_winresult() == *other } }

impl<O, E: PartialEq<Kind>> PartialEq<Kind> for Result<O, E> { fn eq(&self, other: &Kind       ) -> bool { match self.as_ref() { Ok(_) => false, Err(e) => *e == *other } } }
impl<O, E: PartialEq<Kind>> PartialEq<Result<O, E>> for Kind { fn eq(&self, other: &Result<O, E>) -> bool { match other.as_ref() { Ok(_) => false, Err(e) => *e == *self } } }



/// Invalid `user_index` or [`Flag`] passed to an XInput function.
pub const BAD_ARGUMENTS : Kind = Kind::new(ERROR::BAD_ARGUMENTS);

/// XAudio2 device IDs are too large (> 4096 characters) for [`get_audio_device_ids`] to read onto the stack.
pub const BUFFER_TOO_SMALL : Kind = Kind::new(ERROR::BUFFER_TOO_SMALL);

/// COM not initialized.  Only observed being returned from `xinputuap.dll`.
pub const CO_E_NOTINITIALIZED : Kind = Kind::from_u32(winresult::CO::E_NOTINITIALIZED.to_u32());

/// No gamepad is connected for `user_index`.  N.B. this can be unreliable for [`get_audio_device_ids`].
pub const DEVICE_NOT_CONNECTED : Kind = Kind::new(ERROR::DEVICE_NOT_CONNECTED);

/// A corresponding XInput function was missing, or XInput itself couldn't be loaded.
pub const INVALID_FUNCTION : Kind = Kind::new(ERROR::INVALID_FUNCTION);
