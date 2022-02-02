#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbasistype)\]
/// D3DBASISTYPE
///
/// Defines the basis type of a high-order patch surface.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct BasisType(D3DBASISTYPE);
pub use BasisType as Basis;

enumish! { Basis => D3DBASISTYPE; Bezier, BSpline, CatmullRom }

#[allow(non_upper_case_globals)] impl Basis { // These are enum-like
    pub const Bezier        : Basis = Basis(D3DBASIS_BEZIER); // 0
    pub const BSpline       : Basis = Basis(D3DBASIS_BSPLINE);
    pub const CatmullRom    : Basis = Basis(D3DBASIS_CATMULL_ROM);
}

#[doc(hidden)] impl Basis {
    pub const BEZIER        : Basis = Basis(D3DBASIS_BEZIER);
    pub const BSPLINE       : Basis = Basis(D3DBASIS_BSPLINE);
    pub const CATMULL_ROM   : Basis = Basis(D3DBASIS_CATMULL_ROM);
}

impl Default for Basis {
    fn default() -> Self { Basis::Bezier } // 0
}

//#cpp2rust D3DBASISTYPE            = d3d::BasisType
//#cpp2rust D3DBASIS_BEZIER         = d3d::Basis::Bezier
//#cpp2rust D3DBASIS_BSPLINE        = d3d::Basis::BSpline
//#cpp2rust D3DBASIS_CATMULL_ROM    = d3d::Basis::CatmullRom
