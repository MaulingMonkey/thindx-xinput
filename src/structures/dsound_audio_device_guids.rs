#[cfg(doc)] use crate::*;
use winapi::shared::guiddef::GUID;
use bytemuck::*;

use core::cmp::*;
use core::fmt::{self, Debug, Display, Formatter};
use core::hash::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid)\]
/// DirectSound audio device GUID retrieved with [`get_dsound_audio_device_guids`]
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct DSoundAudioDeviceGuid(pub(crate) GUID);

impl DSoundAudioDeviceGuid {
    /// `{00000000-0000-0000-0000-000000000000}` - the "null" guid
    pub const NULL : Self = Self(GUID { Data1: 0, Data2: 0, Data3: 0, Data4: [0; 8] });
}

unsafe impl Pod         for DSoundAudioDeviceGuid {}
unsafe impl Zeroable    for DSoundAudioDeviceGuid {}
impl Default            for DSoundAudioDeviceGuid { fn default() -> Self { Self::zeroed() } }

impl Debug              for DSoundAudioDeviceGuid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.fmt_impl(fmt) } }
impl Display            for DSoundAudioDeviceGuid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.fmt_impl(fmt) } }

impl AsRef<Self>        for DSoundAudioDeviceGuid { fn as_ref(&    self) -> &    Self { self } }
impl AsMut<Self>        for DSoundAudioDeviceGuid { fn as_mut(&mut self) -> &mut Self { self } }

impl Eq                 for DSoundAudioDeviceGuid {}
impl PartialEq          for DSoundAudioDeviceGuid { fn eq(&self, other: &Self) -> bool { self.as_bytes() == other.as_bytes() } }
impl Ord                for DSoundAudioDeviceGuid { fn cmp(&self, other: &Self) -> Ordering { self.as_bytes().cmp(other.as_bytes()) } }
impl PartialOrd         for DSoundAudioDeviceGuid { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.as_bytes().partial_cmp(other.as_bytes()) } }
impl Hash               for DSoundAudioDeviceGuid { fn hash<H: Hasher>(&self, state: &mut H) { self.as_bytes().hash(state) } }

impl DSoundAudioDeviceGuid {
    fn fmt_impl(&self, fmt: &mut Formatter) -> fmt::Result {
        let GUID { Data1: a, Data2: b, Data3: c, Data4: [d0, d1, d2, d3, d4, d5, d6, d7] } = self.0;
        write!(fmt, "{{{a:08X}-{b:04X}-{c:04X}-{d0:02X}{d1:02X}-{d2:02X}{d3:02X}{d4:02X}{d5:02X}{d6:02X}{d7:02X}}}")
    }

    fn as_bytes(&self) -> &[u8] { bytemuck::bytes_of(self) }
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetdsoundaudiodeviceguids)\]
/// DirectSound audio device GUIDs retrieved with [`get_dsound_audio_device_guids`]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Pod, Zeroable)]
#[repr(C)] pub struct DSoundAudioDeviceGuids {
    /// GUID of the headset sound rendering device.
    pub dsound_render_guid:     DSoundAudioDeviceGuid,

    /// GUID of the headset sound capture device.
    pub dsound_capture_guid:    DSoundAudioDeviceGuid,
}



#[test] fn test_traits_for_coverage() {
    let _audio = DSoundAudioDeviceGuids::default();
    let _audio = DSoundAudioDeviceGuids::zeroed();
    let _audio = _audio.clone();
    dbg!(_audio);
}

#[test] fn test_display() {
    assert_eq!("{6B29FC40-CA47-1067-B31D-00DD010662DA}", DSoundAudioDeviceGuid(GUID { Data1: 0x6B29FC40, Data2: 0xCA47, Data3: 0x1067, Data4: *b"\xB3\x1D\x00\xDD\x01\x06\x62\xDA" }).to_string());
}

//#cpp2rust GUID                = DSoundAudioDeviceGuid
//#cpp2rust REFGUID             = &DSoundAudioDeviceGuid
//#cpp2rust GUID_NULL           = DSoundAudioDeviceGuid::NULL
//#cpp2rust InlineIsEqualGUID   = DSoundAudioDeviceGuid::eq
//#cpp2rust IsEqualGUID         = DSoundAudioDeviceGuid::eq
