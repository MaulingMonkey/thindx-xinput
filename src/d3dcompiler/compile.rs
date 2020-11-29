use crate::*;

use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::os::windows::ffi::*;
use std::path::*;
use std::ptr::*;



/// { code: [ReadOnlyBlob], errors: Option&lt;[ReadOnlyBlob]&gt; } returned by [D3DCompiler::compile]/[compile2](D3DCompiler::compile2)
pub struct CompileResult {
    pub shader:     ReadOnlyBlob,
    pub errors:     Option<ReadOnlyBlob>,
}



/// { kind: [ErrorKind], shader: Option&lt;[ReadOnlyBlob]&gt;, errors: Option&lt;[ReadOnlyBlob]&gt; }
pub struct CompileError {
    pub kind:       ErrorKind,
    pub shader:     Option<ReadOnlyBlob>, // may or may not have generated a shader blob despite errors
    pub errors:     Option<ReadOnlyBlob>, // may or may not have generated error messages depending on the error kind
}

impl CompileError {
    pub fn errors_utf8_lossy(&self) -> Option<Cow<str>> {
        let errors = self.errors.as_ref()?.get_buffer();
        Some(match String::from_utf8_lossy(errors) {
            Cow::Borrowed(s) => Cow::Borrowed(s.trim_end_matches("\0")),
            Cow::Owned(mut s) => {
                while s.ends_with("\0") { s.pop(); }
                Cow::Owned(s)
            },
        })
    }
}

impl From<ErrorKind> for CompileError {
    fn from(kind: ErrorKind) -> Self { Self { kind, shader: None, errors: None } }
}

impl std::error::Error for CompileError {}

impl Debug for CompileError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("CompileError")
            .field("kind", &self.kind)
            .field("shader", &self.shader.as_ref().map(|_| ..))
            .field("errors", &self.errors_utf8_lossy())
            .finish()
    }
}

impl Display for CompileError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Error compiling shader: {:?}", self.kind)?;
        if let Some(errors) = self.errors_utf8_lossy() {
            writeln!(fmt, "\n{}", errors)?;
        }
        Ok(())
    }
}



/// <h1 id="compile" class="section-header"><a href="#compile">Compile & Preprocess HLSL to Bytecode</a></h1>
impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompilefromfile)\]
    /// D3DCompileFromFile
    ///
    /// > **Note:** You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.
    /// > Refer to the section, "Compiling shaders for UWP", in the remarks for [compile2](Self::compile2).
    ///
    /// Compiles Microsoft High Level Shader Language (HLSL) code into bytecode for a given target.
    ///
    /// ### Arguments
    /// *   `file_name`     - The shader file to compile
    /// *   `defines`       - An optional array of defines.  Use [None] if no extra defines are desired.
    /// *   `include`       - An optional interface for dispatching `#include`s.
    ///                       Use [None] if `#include`s should not be supported.
    ///                       Use [StandardFileInclude] if you want to resolve `#include`s relative to `source_name`.
    /// *   `entrypoint`    - An optional entrypoint such as `Some("main")`.  Ignored if `target` is `fx_*`.
    /// *   `target`        - A target shader profile such as `ps_3_0`, `vs_5_0`, `fx_4_0`, etc.
    /// *   `flags1`        - [Compile]::\* constants.
    /// *   `flags2`        - [CompileEffect]::\* constants.
    ///
    /// ### Returns
    /// *   Ok([CompileResult] { code: [ReadOnlyBlob], errors: [Option]&lt;[ReadOnlyBlob]&gt; })
    /// *   Err([CompileError]) where `error.kind` ==
    ///     *   [ErrorKind::MISSING_DLL_EXPORT]     - `d3dcompiler_4?.dll` and earlier
    ///     *   [D3DERR::INVALIDCALL]               - on invalid parameters such as nonexistant `target`s
    ///     *   [E::FAIL]                           - if the shader failed to compile
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let pixel_shader = compiler.compile_from_file(
    ///     r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    ///
    /// let vertex_shader = compiler.compile_from_file(
    ///     r"test\data\basic.hlsl", None, StandardFileInclude, "vs_main", "vs_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    ///
    /// // TODO: defines/includes examples
    /// ```
    ///
    /// <div class="note"><b>Note:</b> You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.</div>
    pub fn compile_from_file<'s>(
        &self,
        file_name:      impl AsRef<Path>,
        defines:        impl AsShaderMacros,
        include:        impl AsID3DInclude,
        entrypoint:     impl TryIntoAsOptCStr,
        target:         impl TryIntoAsCStr,
        flags1:         impl Into<Compile>,
        flags2:         impl Into<CompileEffect>,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DCompileFromFile.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;
        let defines     = defines.as_shader_macros()?;

        let file_name = file_name.as_ref().as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();

        let entrypoint  = entrypoint.try_into().map_err(|e| CompileError { kind: e, shader: None, errors: None })?;
        let target      = target    .try_into().map_err(|e| CompileError { kind: e, shader: None, errors: None })?;
        let entrypoint  = entrypoint.as_opt_cstr();
        let target      = target    .as_cstr();

        let include     = include.as_id3dinclude();
        let flags1      = flags1.into().into();
        let flags2      = flags2.into().into();

        let mut shader = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { f(
            file_name.as_ptr(),
            defines, include, entrypoint, target,
            flags1, flags2, &mut shader, &mut errors,
        )};
        ErrorKind::check(hr).map_err(|kind| CompileError {
            kind,
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })?;

        Ok(CompileResult {
            shader: unsafe { ReadOnlyBlob::from_raw(shader as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile)\]
    /// D3DCompile
    ///
    /// Compile HLSL code or an effect file into bytecode for a given target.
    ///
    /// ### Arguments
    /// *   `src_data`      - The shader source code
    /// *   `source_name`   - An optional shader name such as `Some("myshader.hlsl")` for debug purpouses.
    /// *   `defines`       - An optional array of defines.  Use [None] if no extra defines are desired.
    /// *   `include`       - An optional interface for dispatching `#include`s.
    ///                       Use [None] if `#include`s should not be supported.
    ///                       Use [StandardFileInclude] if you want to resolve `#include`s relative to `source_name`.
    /// *   `entrypoint`    - An optional entrypoint such as `Some("main")`.  Ignored if `target` is `fx_*`.
    /// *   `target`        - A target shader profile such as `ps_3_0`, `vs_5_0`, `fx_4_0`, etc.
    /// *   `flags1`        - [Compile]::\* constants.
    /// *   `flags2`        - [CompileEffect]::\* constants.
    ///
    /// ### Returns
    /// *   Ok([CompileResult] { code: [ReadOnlyBlob], errors: [Option]&lt;[ReadOnlyBlob]&gt; })
    /// *   Err([CompileError]) where `error.kind` ==
    ///     *   [ErrorKind::MISSING_DLL_EXPORT]     - `d3dcompiler_39.dll` and earlier
    ///     *   [D3DERR::INVALIDCALL]               - on invalid parameters such as nonexistant `target`s
    ///     *   [E::FAIL]                           - if the shader failed to compile
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let basic_hlsl = std::fs::read(r"test\data\basic.hlsl").unwrap();
    ///
    /// let pixel_shader = compiler.compile(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    ///
    /// let vertex_shader = compiler.compile(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0",
    ///     Compile::Debug, CompileEffect::None,
    /// ).unwrap();
    /// ```
    ///
    /// <div class="note"><b>Note:</b> This fn was introduced by d3dcompiler_40.dll, and is unavailable in earlier versions.</div>
    #[cfg_attr(not(d3dcompiler="40"), deprecated(note = "D3DCompiler::compile wasn't added until d3dcompiler_40.dll"))]
    pub fn compile<'s>(
        &self,
        src_data:       &[u8],
        source_name:    impl TryIntoAsOptCStr,
        defines:        impl AsShaderMacros,
        include:        impl AsID3DInclude,
        entrypoint:     impl TryIntoAsOptCStr,
        target:         impl TryIntoAsCStr,
        flags1:         impl Into<Compile>,
        flags2:         impl Into<CompileEffect>,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DCompile.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;
        let defines     = defines.as_shader_macros()?;

        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let source_name = source_name   .try_into().map_err(|e| CompileError { kind: e, shader: None, errors: None })?;
        let entrypoint  = entrypoint    .try_into().map_err(|e| CompileError { kind: e, shader: None, errors: None })?;
        let target      = target        .try_into().map_err(|e| CompileError { kind: e, shader: None, errors: None })?;
        let source_name = source_name   .as_opt_cstr();
        let entrypoint  = entrypoint    .as_opt_cstr();
        let target      = target        .as_cstr();

        let include     = include.as_id3dinclude();
        let flags1      = flags1.into().into();
        let flags2      = flags2.into().into();

        let mut shader = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { f(
            src_data.as_ptr().cast(), src_data.len(),
            source_name, defines, include, entrypoint, target,
            flags1, flags2, &mut shader, &mut errors,
        )};
        ErrorKind::check(hr).map_err(|kind| CompileError {
            kind,
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })?;

        Ok(CompileResult {
            shader: unsafe { ReadOnlyBlob::from_raw(shader as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile2)\]
    /// D3DCompile2
    ///
    /// Compiles Microsoft High Level Shader Language (HLSL) code into bytecode for a given target.
    ///
    /// ### Arguments
    /// *   `src_data`      - The shader source code
    /// *   `source_name`   - An optional shader name such as `Some("myshader.hlsl")` for debug purpouses.
    /// *   `defines`       - An optional array of defines.  Use [None] if no extra defines are desired.
    /// *   `include`       - An optional interface for dispatching `#include`s.
    ///                       Use [None] if `#include`s should not be supported.
    ///                       Use [StandardFileInclude] if you want to resolve `#include`s relative to `source_name`.
    /// *   `entrypoint`    - An optional entrypoint such as `Some("vs_main")`.  Ignored if `target` is `"fx_*"`.
    /// *   `target`        - A target shader profile such as `Some("ps_3_0")`, `Some("vs_5_0")`, `Some("fx_4_0")`, etc.
    /// *   `flags1`        - [Compile]::\* constants.
    /// *   `flags2`        - [CompileEffect]::\* constants.
    /// *   `secondary_data_flags`  - [CompileSecData]::\* constants.
    /// *   `secondary_data`    - A pointer to secondary data. If you don't pass secondary data, set to [None].
    ///
    /// ### Returns
    /// *   Ok([CompileResult] { code: [ReadOnlyBlob], errors: [Option]&lt;[ReadOnlyBlob]&gt; })
    /// *   Err([CompileError]) where `error.kind` ==
    ///     *   [ErrorKind::MISSING_DLL_EXPORT]     - `d3dcompiler_4?.dll` and earlier
    ///     *   [D3DERR::INVALIDCALL]               - on invalid parameters such as nonexistant `target`s
    ///     *   [E::FAIL]                           - if the shader failed to compile
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let basic_hlsl = std::fs::read(r"test\data\basic.hlsl").unwrap();
    ///
    /// let pixel_shader = compiler.compile2(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None, CompileSecData::None, None,
    /// ).unwrap();
    ///
    /// let vertex_shader = compiler.compile2(
    ///     &basic_hlsl, r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0",
    ///     Compile::Debug, CompileEffect::None, CompileSecData::None, None,
    /// ).unwrap();
    /// ```
    ///
    /// ### Remarks
    /// The difference between [compile2](Self::compile2) and [compile](Self::compile) is that [compile2](Self::compile2)
    /// takes some optional parameters (`secondary_data_flags` and `secondary_data`) that can be used to control some
    /// aspects of how bytecode is generated.  Refer to the descriptions of these parameters for more details. There is
    /// no difference otherwise to the efficiency of the bytecode generated between [compile2](Self::compile2) and
    /// [compile](Self::compile).
    pub fn compile2<'s>(
        &self,
        src_data:               &[u8],
        source_name:            impl TryIntoAsOptCStr,
        defines:                impl AsShaderMacros,
        include:                impl AsID3DInclude,
        entrypoint:             impl TryIntoAsOptCStr,
        target:                 impl TryIntoAsCStr,
        flags1:                 impl Into<Compile>,
        flags2:                 impl Into<CompileEffect>,
        secondary_data_flags:   impl Into<CompileSecData>,
        secondary_data:         impl Into<Option<&'s [u8]>>,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DCompile2.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;
        let defines     = defines.as_shader_macros()?;

        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let source_name = source_name   .try_into().map_err(|e| CompileError { kind: e, shader: None, errors: None })?;
        let entrypoint  = entrypoint    .try_into().map_err(|e| CompileError { kind: e, shader: None, errors: None })?;
        let target      = target        .try_into().map_err(|e| CompileError { kind: e, shader: None, errors: None })?;
        let source_name = source_name   .as_opt_cstr();
        let entrypoint  = entrypoint    .as_opt_cstr();
        let target      = target        .as_cstr();

        let include     = include.as_id3dinclude();
        let flags1      = flags1.into().into();
        let flags2      = flags2.into().into();
        let secondary_data_flags = secondary_data_flags.into().into();

        let secondary_data      = secondary_data.into();
        let secondary_data_len  = secondary_data.map_or(0, |sd| sd.len());
        let secondary_data      = secondary_data.map_or(null(), |sd| sd.as_ptr()).cast();

        let mut shader = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { f(
            src_data.as_ptr().cast(), src_data.len(),
            source_name, defines, include, entrypoint, target,
            flags1, flags2, secondary_data_flags, secondary_data, secondary_data_len,
            &mut shader, &mut errors,
        )};
        ErrorKind::check(hr).map_err(|kind| CompileError {
            kind,
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })?;

        Ok(CompileResult {
            shader: unsafe { ReadOnlyBlob::from_raw(shader as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dpreprocess)\]
    /// D3DPreprocess
    ///
    /// Preprocesses uncompiled HLSL code.
    ///
    /// ### Arguments
    /// *   `src_data`      - The shader source code
    /// *   `source_name`   - An optional shader name such as `Some("myshader.hlsl")` for debug purpouses.
    /// *   `defines`       - An optional array of defines.  Use `()` if no extra defines are desired.
    /// *   `include`       - An optional interface for dispatching `#include`s.
    ///                       Use `()` if `#include`s should not be supported.
    ///                       Use [StandardFileInclude] if you want to resolve `#include`s relative to `source_name`.
    ///
    /// ### Returns
    /// *   Err([ErrorKind::MISSING_DLL_EXPORT])    - `d3dcompiler_39.dll` and earlier
    /// *   Ok([CompileResult] { code, errors })
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let basic_hlsl = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// let ps = compiler.preprocess(&basic_hlsl, r"test\data\basic.hlsl", (), None).unwrap();
    /// println!("{}", String::from_utf8_lossy(ps.shader.get_buffer()));
    /// ```
    ///
    /// ### Output
    /// ```hlsl
    /// #line 1 "C:\\local\\thin3dcompiler\\test\\data\\basic.hlsl"
    ///
    ///
    /// struct Vertex {
    ///     float4 position : POSITION0 ;
    ///     float4 color : COLOR0 ;
    /// } ;
    ///
    /// struct VsToPs {
    /// ...
    /// ```
    ///
    /// <div class="note"><b>Note:</b> This fn was introduced by d3dcompiler_40.dll, and is unavailable in earlier versions.</div>
    #[cfg_attr(not(d3dcompiler="40"), deprecated(note = "D3DCompiler::compile wasn't added until d3dcompiler_40.dll"))]
    pub fn preprocess<'s>(
        &self,
        src_data:       &[u8],
        source_name:    impl TryIntoAsOptCStr,
        defines:        impl AsShaderMacros,
        include:        impl AsID3DInclude,
    ) -> Result<CompileResult, CompileError> {
        // Early outs
        let f           = self.D3DPreprocess.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;
        let defines     = defines.as_shader_macros()?;

        // Note: No error checking occurs for internal `\0`s - they will simply terminate the string earlier than expected.
        // Note: We should perhaps reject non-ASCII values instead of allowing UTF8
        let source_name = source_name.try_into().map_err(|e| CompileError { kind: e, shader: None, errors: None })?;
        let source_name = source_name.as_opt_cstr();
        let include     = include.as_id3dinclude();

        let mut shader = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), source_name, defines, include, &mut shader, &mut errors)};
        ErrorKind::check(hr).map_err(|kind| CompileError {
            kind,
            shader: unsafe { ReadOnlyBlob::from_raw_opt(shader as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })?;

        Ok(CompileResult {
            shader: unsafe { ReadOnlyBlob::from_raw(shader as *mut _) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors as *mut _) },
        })
    }
}
