# Design Decisions

#### Why don't you link `xinput.lib`?  Or `kind = "raw-dylib"` equivalent?

1.  These don't allow for stub/noop fallbacks if the XInput DLLs are missing.
2.  These don't allow good control over which XInput DLLs are used.

You might ask: why would DLL version control matter?  This is best explained by example.  For one bug I encountered:
*   I upgraded [`winit`](https://docs.rs/winit/) from [`"0.26"`](https://docs.rs/winit/0.26/) → [`"0.27"`](https://docs.rs/winit/0.27/), which in turn updated [`windows-sys`](https://docs.rs/windows-sys/).
*   This provided it's own import lib, linking XInput symbols against `XInputUap.dll`, introducing [a crash bug on exit](https://github.com/microsoft/win32metadata/issues/1274).
*   This happened despite not using `features = ["Win32_UI_Input_XboxController"]`.
*   Said linkage "won" the "race" to resolve XInput symbols in code entirely unrelated to `windows-sys`.
*   [`kind = "raw-dylib"`](https://doc.rust-lang.org/reference/items/external-blocks.html#dylib-versus-raw-dylib) does not avoid the static linkage race.  I've tried it.  Conflicting definitions will be merged.
