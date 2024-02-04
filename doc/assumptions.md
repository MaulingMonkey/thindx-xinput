# Assumptions

This crate attempts to use [various `XInput{...}.dll` versions](versions) via:
*   [`LoadLibrary`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)
*   [`GetModuleHandleEx`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandleexw)

These DLLs are assumped to be, if not authentic Microsoft XInput DLLs, at least similarly behaving DLLs:
*   Tolerant of invalid arguments besides dangling pointers (returning an error code counts.)
*   Tolerant of multithreading.
*   Same function signatures.
*   Same layouts for `XINPUT_*` structs.
*   Exhibiting defined behavior.
*   ...

Technically, this is unsound &mdash; nothing is stopping the user from suffering undefined behavior via:
*   Replacing `XInput1_4.dll` with `evil_crimes.dll` (arguably beyond the scope of the Rust Abstract Machine and thus "OK")
*   [`XInputUap.dll`'s on-exit crash bugs](https://github.com/microsoft/win32metadata/issues/1274) (Microsoft's bug, predictable, but C++ Undefined Behavior is not exactly "OK")
