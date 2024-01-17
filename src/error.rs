use crate::*;
use crate::error_macros::FnContext;

use winresult::{ErrorCode, HResultError};

use std::fmt::{self, Debug, Display, Formatter};



/// An error returned by some XInput function (or wrapper thereof.)
#[derive(Clone)]
pub struct Error(pub(crate) &'static FnContext, pub(crate) ErrorKind);

impl Error {
    pub(crate) fn method(&self) -> &'static str { self.0.directx_method.unwrap_or(self.0.thindx_method) }

    /// Returns the [ErrorKind] of the error
    pub fn kind(&self) -> ErrorKind { self.1 }
}

impl Debug   for Error { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "Error({:?}, {:?})", self.method(), self.1) } }
impl Display for Error { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{} failed, returning {:?}", self.method(), self.1) } }

impl std::error::Error for Error {}
impl From<Error> for std::io::Error { fn from(err: Error) -> Self { std::io::Error::new(std::io::ErrorKind::Other, err) } }

impl PartialEq<Error> for ErrorCode     { fn eq(&self, other: &Error        ) -> bool { other.kind() == *self } }
impl PartialEq<Error> for HResultError  { fn eq(&self, other: &Error        ) -> bool { other.kind() == *self } }
impl PartialEq<ErrorCode    > for Error { fn eq(&self, other: &ErrorCode    ) -> bool { self.kind() == *other } }
impl PartialEq<HResultError > for Error { fn eq(&self, other: &HResultError ) -> bool { self.kind() == *other } }
