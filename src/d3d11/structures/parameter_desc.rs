use crate::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_parameter_desc)\]
/// D3D11_PARAMETER_DESC
///
/// &amp;\[[u8]\] equivalent that's ABI-compatible with some D3D APIs
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct ParameterDesc<'s> {
    name:                   Option<CStrPtr<'s>>,
    semantic_name:          Option<CStrPtr<'s>>,
    r#type:                 ShaderVariableType,
    class:                  ShaderVariableClass,
    rows:                   u32,
    columns:                u32,
    interpolation_mode:     InterpolationMode,
    flags:                  ParameterFlags,
    first_in_register:      u32,
    first_in_component:     u32,
    first_out_register:     u32,
    first_out_component:    u32,
}

test_layout! { ParameterDesc => unsafe D3D11_PARAMETER_DESC {
    name                    => Name,
    semantic_name           => SemanticName,
    r#type                  => Type,
    class                   => Class,
    rows                    => Rows,
    columns                 => Columns,
    interpolation_mode      => InterpolationMode,
    flags                   => Flags,
    first_in_register       => FirstInRegister,
    first_in_component      => FirstInComponent,
    first_out_register      => FirstOutRegister,
    first_out_component     => FirstOutComponent,
}}