#![allow(dead_code)] // TODO: remove

use crate::*;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexdeclaration9)\]
/// Describes the layout of the contents of a [VertexBuffer]
#[derive(Clone)] #[repr(transparent)]
pub struct VertexDeclaration(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DVertexDeclaration9>);



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexdeclaration9)\]
/// IDirect3DVertexDeclaration9 extension methods
///
/// ### Methods
///
/// | thin3d9                                                       | docs.microsoft.com    | Description |
/// | ------------------------------------------------------------- | --------------------- | ----------- |
/// | [get_declaration_size](Self::get_declaration_size)            | [GetDeclaration]      | Get the number of elements in this vertex declaration, including the [VertexElement::END]
/// | [get_declaration_inplace](Self::get_declaration_inplace)      | [GetDeclaration]      | Gets the elements in this vertex declaration, including the [VertexElement::END]
/// | [get_declaration](Self::get_declaration)                      | [GetDeclaration]      | Gets the elements in this vertex declaration, including the [VertexElement::END]
/// | [get_device](Self::get_device)                                | [GetDevice]           | Gets the current device.
///
/// [GetDeclaration]:   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexdeclaration9-getdeclaration
/// [GetDevice]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexdeclaration9-getdevice
///
pub trait IDirect3DVertexDeclaration9Ext : private::Sealed {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexdeclaration9-getdeclaration)\]
    /// IDirect3DVertexDeclaration9::GetDeclaration
    ///
    /// Get the number of elements in this vertex declaration, including the [VertexElement::END]
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the device is a pure device?
    /// *   Ok([u32])               The number of elements in this vertex declaration, including the [VertexElement::END]
    fn get_declaration_size(&self) -> Result<u32, MethodError> {
        let mut num_elements = 0;
        let hr = unsafe { self.as_winapi().GetDeclaration(null_mut(), &mut num_elements) };
        MethodError::check("IDirect3DVertexDeclaration9::GetDeclaration", hr)?;
        Ok(num_elements)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexdeclaration9-getdeclaration)\]
    /// IDirect3DVertexDeclaration9::GetDeclaration
    ///
    /// Gets the elements in this vertex declaration, including the [VertexElement::END]
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the device is a pure device?
    /// *   [D3DERR::INVALIDCALL]   If `elements` is too small to contain the result
    /// *   Ok(&[[VertexElement]])                  If `elements` was successfully written to, including the [VertexElement::END]
    fn get_declaration_inplace<'e>(&self, elements: &'e mut [VertexElement]) -> Result<&'e [VertexElement], MethodError> {
        let mut num_elements = self.get_declaration_size()?;
        if num_elements as usize > elements.len() { return Err(MethodError("VertexDeclaration::get_declaration_inplace", D3DERR::INVALIDCALL)); }
        let hr = unsafe { self.as_winapi().GetDeclaration(null_mut(), &mut num_elements) };
        MethodError::check("IDirect3DVertexDeclaration9::GetDeclaration", hr)?;
        Ok(&elements[0..(num_elements as usize)])
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexdeclaration9-getdeclaration)\]
    /// IDirect3DVertexDeclaration9::GetDeclaration
    ///
    /// Gets the elements in this vertex declaration, including the [VertexElement::END]
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]               If the device is a pure device?
    /// *   Ok(Vec&lt;[VertexElement]&gt;)      The elements of this vertex declaration, including the [VertexElement::END]
    fn get_declaration(&self) -> Result<Vec<VertexElement>, MethodError> {
        let mut num_elements = self.get_declaration_size()?;
        let mut v = vec![VertexElement::default(); num_elements as usize];
        let hr = unsafe { self.as_winapi().GetDeclaration(v.as_mut_ptr().cast(), &mut num_elements) };
        debug_assert!(v.len() == num_elements as usize); // size didn't change, right?
        MethodError::check("IDirect3DVertexDeclaration9::GetDeclaration", hr)?;
        Ok(v)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dvertexdeclaration9-getdevice)\]
    /// IDirect3DVertexDeclaration9::GetDevice
    ///
    /// Gets the current device.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the device is a pure device?
    /// *   Ok([Device])
    fn get_device(&self) -> Result<Device, MethodError> {
        let mut device = null_mut();
        let hr = unsafe { self.as_winapi().GetDevice(&mut device) };
        MethodError::check("IDirect3DVertexDeclaration9::GetDevice", hr)?;
        Ok(unsafe { Device::from_raw(device) })
    }
}

impl<T: private::Sealed> IDirect3DVertexDeclaration9Ext for T {}

mod private {
    use winapi::shared::d3d9::IDirect3DVertexDeclaration9;
    pub unsafe trait Sealed                                         { fn as_winapi(&self) -> &IDirect3DVertexDeclaration9; }
    unsafe impl Sealed for mcom::Rc<IDirect3DVertexDeclaration9>    { fn as_winapi(&self) -> &IDirect3DVertexDeclaration9 { &**self } }
    unsafe impl Sealed for super::VertexDeclaration                 { fn as_winapi(&self) -> &IDirect3DVertexDeclaration9 { &*self.0 } }
}

// #[test] fn create_vertex_declaration() {} // TODO

// TODO: test coverage, examples