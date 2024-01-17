# Alternative APIs

\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-game-controller-apis-portal)\] **XInput**
*   ✔️ Supports Xbox 360 controllers with separate triggers out of the box
*   ❌ Supports little else
*   FFI: [`winapi::um::xinput`](https://docs.rs/winapi/0.3/winapi/um/xinput/)<br>
*   FFI: [`windows    ::Win32::UI::Input::XboxController`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Input/XboxController/)<br>
*   FFI: [`windows_sys::Win32::UI::Input::XboxController`](https://docs.rs/windows-sys/latest/windows_sys/Win32/UI/Input/XboxController/)<br>

\[[microsoft.com](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee416842(v=vs.85))\] **DirectInput**
*   ✔️ Supports joysticks with many more buttons and axises than XInput.
*   ✔️ Leverages Windows's built in support for configuring idle positions, deadzones.
*   ⚠️ Older, "deprecated" in favor of XInput.
*   ❌ Xbox 360 controllers map both triggers to a single axis in DirectInput.
*   [Comparison of XInput and DirectInput features](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-and-directinput)
    discusses how to use a hybrid approach of XInput for 360 controllers and DirectInput for non-XInput devices
*   FFI: [`winapi::um::dinput`](https://docs.rs/winapi/0.3/winapi/um/dinput/)
*   FFI: [`windows    ::Win32::Devices::HumanInterfaceDevice`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Devices/HumanInterfaceDevice/)
*   FFI: [`windows_sys::Win32::Devices::HumanInterfaceDevice`](https://docs.rs/windows-sys/latest/windows_sys/Win32/Devices/HumanInterfaceDevice/)

\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/inputdev/about-raw-input)\] **RawInput**
*   ✔️ Supports a *lot* of stuff you might not be able to support in nearly any other fashion.
*   ⚠️ Parse practically everything yourself, as if you were writing your own usermode driver.
*   ❌ Xbox 360 controllers map both triggers to a single axis ala DirectInput.
*   FFI: [`winapi::um::winuser::*RAW*`](https://docs.rs/winapi/0.3.9/winapi/um/winuser/?search=winuser%3A%3ARAW)
*   FFI: [`windows    ::Win32::UI::Input::RAW*`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Input/)
*   FFI: [`windows_sys::Win32::UI::Input::RAW*`](https://docs.rs/windows-sys/latest/windows_sys/Win32/UI/Input/)

\[[microsoft.com](https://learn.microsoft.com/en-us/windows/uwp/gaming/input-for-games)\] **`Windows.Gaming.Input` (UWP)**
*   ✔️ Supports Xbox One trigger rumble (XInput only supports base controller rumble)
*   ⚠️ Can't recieve input through UWP when the app/window is not active (useful for dev cruft.)
*   ⚠️ No Windows 7 support?
*   FFI: [`windows::Gaming::Input`](https://microsoft.github.io/windows-docs-rs/doc/windows/Gaming/Input/)

\[[microsoft.com](https://learn.microsoft.com/en-us/gaming/gdk/_content/gc/input/overviews/input-overview)\] **GameInput**
*   ⚠️ Modern micro-COM API to superceede all others?
*   ⚠️ *Eventual* [Windows 7](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-game-controller-apis-portal) support?
*   ❌ Not yet available as of Windows SDK 10.0.22621.0 ?
*   FFI: N/A
