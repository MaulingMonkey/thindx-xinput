//! Use [D3DCompiler::compile] and friends to compile HLSL to bytecode
#![allow(unused_variables)]
use thindx::*;

fn main() {
    let compiler = D3DCompiler::new(47).unwrap();

    // Option A:  compile_from_file
    let pixel_shader  = compiler.compile_from_file(r"test\data\basic.hlsl",   None, None,                "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let vertex_shader = compiler.compile_from_file(r"test\data\basic.hlsl",   None, StandardFileInclude, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let library       = compiler.compile_from_file(r"test\data\library.hlsl", None, None,                (),       "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();

    // Option B:  compile
    let basic_hlsl   : &[u8] = include_bytes!(r"..\test\data\basic.hlsl");
    let library_hlsl : &[u8] = include_bytes!(r"..\test\data\library.hlsl");
    let pixel_shader  = compiler.compile(basic_hlsl,   (),                      None, None,                "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let vertex_shader = compiler.compile(basic_hlsl,   r"test\data\basic.hlsl", None, StandardFileInclude, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let library       = compiler.compile(library_hlsl, r"library.hlsl",         None, None,                (),       "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();

    // Option C:  compile2
    let pixel_shader  = compiler.compile2(basic_hlsl,   (), None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
    let vertex_shader = compiler.compile2(basic_hlsl,   (), None, None, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
    let library       = compiler.compile2(library_hlsl, (), None, None, (),       "lib_5_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();

    // TODO: show ID3DInclude usage
    // TODO: show defines usage
    // TODO: show effects usage?
}

// TODO: include hlsl in docs (add an xtask directive?)