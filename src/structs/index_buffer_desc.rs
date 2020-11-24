use crate::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/D3DINDEXBUFFER_DESC)\]
/// D3DINDEXBUFFER_DESC
///
/// Describes an index buffer
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct IndexBufferDesc {
    pub format:     Format,
    pub r#type:     ResourceType,
    pub usage:      Usage,
    pub pool:       Pool,
    pub size:       u32,
}

impl Deref    for IndexBufferDesc { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DINDEXBUFFER_DESC; }
impl DerefMut for IndexBufferDesc { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DINDEXBUFFER_DESC> for IndexBufferDesc { fn from(value: D3DINDEXBUFFER_DESC) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<IndexBufferDesc> for D3DINDEXBUFFER_DESC { fn from(value: IndexBufferDesc    ) -> Self { unsafe { std::mem::transmute(value) } } }

test_layout! { IndexBufferDesc => unsafe D3DINDEXBUFFER_DESC { format => Format, r#type => Type, usage => Usage, pool => Pool, size => Size } }
