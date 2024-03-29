macro_rules! fn_param_error { ( $param:ident, $kind:expr ) => { $crate::Error(&$crate::error_macros::FnContext { parameter: Some(stringify!($param)), .._THINDX_FN_CONTEXT }, ($kind).into()) } }

/// Annotate a Rust => C++ function mapping.
///
/// This is an alternative to `//#cpp2rust Cpp = Rust` comments, and also provides a constant for other `fn_*!` macros to read.
///
/// ### Usage
/// ```ignore
/// fn_context!(Rust::method => ICpp::Method);
/// ```
macro_rules! fn_context {
    ( $thindx:path ) => {
        const _THINDX_FN_CONTEXT : $crate::error_macros::FnContext = $crate::error_macros::FnContext { // Ensure it's evaluated at compile time
            directx_method: None,
            thindx_method:  stringify!($thindx),
            parameter:      None,

            module_path:    std::module_path!(),
            file:           std::file!(),
            line:           std::line!(),
            column:         std::column!(),
        };
    };
    ( $thindx:path => $directx:path ) => {
        const _THINDX_FN_CONTEXT : $crate::error_macros::FnContext = $crate::error_macros::FnContext { // Ensure it's evaluated at compile time
            directx_method: Some(stringify!($directx)),
            thindx_method:  stringify!($thindx),
            parameter:      None,

            module_path:    std::module_path!(),
            file:           std::file!(),
            line:           std::line!(),
            column:         std::column!(),
        };
    };
}

#[allow(dead_code)] // not all fields yet used
pub(crate) struct FnContext {
    pub directx_method: Option<&'static str>,
    pub thindx_method:  &'static str,
    pub parameter:      Option<&'static str>,

    pub module_path:    &'static str,
    pub file:           &'static str,
    pub line:           u32,
    pub column:         u32,
}



#[test] fn asdf() {
    fn_context! { thindx::method => Method };
}
