#![warn(clippy::undocumented_unsafe_blocks)]

use bytemuck::{Pod, Zeroable};

use winapi::shared::guiddef::GUID;

use std::cmp::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid)\]
/// A 128-bit identifier used for COM interfaces, COM class objects, and various other purpouses.
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Guid(GUID);

impl Guid {
    /// `{00000000-0000-0000-0000-000000000000}` - the "null" guid
    pub const NULL : Self = Self(GUID { Data1: 0, Data2: 0, Data3: 0, Data4: [0; 8] });
}

unsafe impl Pod         for Guid {}
unsafe impl Zeroable    for Guid {}
impl Default            for Guid { fn default() -> Self { Self::zeroed() } }

impl Debug              for Guid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.fmt_impl(fmt) } }
impl Display            for Guid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.fmt_impl(fmt) } }

impl AsRef<Guid> for Guid { fn as_ref(&    self) -> &    Guid {      self } }
impl AsMut<Guid> for Guid { fn as_mut(&mut self) -> &mut Guid {      self } }
impl AsRef<GUID> for Guid { fn as_ref(&    self) -> &    GUID { &    self.0 } }
impl AsMut<GUID> for Guid { fn as_mut(&mut self) -> &mut GUID { &mut self.0 } }
impl AsRef<Guid> for GUID { fn as_ref(&    self) -> &    Guid { unsafe { std::mem::transmute(self) } } }
impl AsMut<Guid> for GUID { fn as_mut(&mut self) -> &mut Guid { unsafe { std::mem::transmute(self) } } }
impl From<GUID> for Guid { fn from(guid: GUID) -> Self { Self(guid) } }
impl From<Guid> for GUID { fn from(guid: Guid) -> Self { guid.0 } }

impl Eq                 for Guid {}
impl PartialEq          for Guid { fn eq(&self, other: &Self) -> bool { self.as_bytes() == other.as_bytes() } }
impl Ord                for Guid { fn cmp(&self, other: &Self) -> Ordering { self.as_bytes().cmp(other.as_bytes()) } }
impl PartialOrd         for Guid { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.as_bytes().partial_cmp(other.as_bytes()) } }
impl Hash               for Guid { fn hash<H: Hasher>(&self, state: &mut H) { self.as_bytes().hash(state) } }

impl Guid {
    fn fmt_impl(&self, fmt: &mut Formatter) -> fmt::Result {
        let GUID { Data1: a, Data2: b, Data3: c, Data4: [d0, d1, d2, d3, d4, d5, d6, d7] } = self.0;
        write!(fmt, "{{{a:08X}-{b:04X}-{c:04X}-{d0:02X}{d1:02X}-{d2:02X}{d3:02X}{d4:02X}{d5:02X}{d6:02X}{d7:02X}}}")
    }

    fn as_bytes(&self) -> &[u8] { bytemuck::bytes_of(self) }
}

#[test] fn test_display() {
    assert_eq!("{6B29FC40-CA47-1067-B31D-00DD010662DA}", Guid(GUID { Data1: 0x6B29FC40, Data2: 0xCA47, Data3: 0x1067, Data4: *b"\xB3\x1D\x00\xDD\x01\x06\x62\xDA" }).to_string());
}

//#cpp2rust GUID                = Guid
//#cpp2rust REFGUID             = &Guid
//#cpp2rust GUID_NULL           = Guid::NULL
//#cpp2rust InlineIsEqualGUID   = Guid::eq
//#cpp2rust IsEqualGUID         = Guid::eq
