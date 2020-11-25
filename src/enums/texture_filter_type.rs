#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturefiltertype)\]
/// D3DTEXTUREFILTERTYPE
///
/// Defines texture filtering modes for a texture stage.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TextureFilterType(D3DTEXTUREFILTERTYPE);
pub type TexF = TextureFilterType;

enumish! { TexF => D3DTEXTUREFILTERTYPE; None, Point, Linear, Anisotropic, PyramidalQuad, GaussianQuad, ConvolutionMono }

#[allow(non_upper_case_globals)] impl TextureFilterType { // These are enum-like
    pub const None              : TextureFilterType = TextureFilterType(D3DTEXF_NONE);
    pub const Point             : TextureFilterType = TextureFilterType(D3DTEXF_POINT);
    pub const Linear            : TextureFilterType = TextureFilterType(D3DTEXF_LINEAR);
    pub const Anisotropic       : TextureFilterType = TextureFilterType(D3DTEXF_ANISOTROPIC);
    pub const PyramidalQuad     : TextureFilterType = TextureFilterType(D3DTEXF_PYRAMIDALQUAD);
    pub const GaussianQuad      : TextureFilterType = TextureFilterType(D3DTEXF_GAUSSIANQUAD);
    pub const ConvolutionMono   : TextureFilterType = TextureFilterType(D3DTEXF_CONVOLUTIONMONO);
}

impl Default for TextureFilterType {
    fn default() -> Self { TextureFilterType::None } // 0
}
