# Crate Features

| Crate Feature     | Description   |
| ------------------| --------------|
| `"undocumented"`  | Enable undocumented XInput APIs which are exported by ordinal such as: <br> [`get_state_ex`], [`power_off_controller`], ...
|                   | **Features for interop with other crates:** (e.g. [`AsRef`], [`AsMut`], [`From`], [`Into`] for all C++ `struct`s)
| `"winapi-0-3"`    | Interop with [`winapi = "0.3"`](https://docs.rs/winapi/0.3/)
| `"winapi-0-2"`    | Interop with [`winapi = "0.2"`](https://docs.rs/winapi/0.2/x86_64-pc-windows-gnu/winapi/) (minus winapi 0.2's broken [`XINPUT_KEYSTROKE`](https://docs.rs/winapi/0.2.8/x86_64-pc-windows-gnu/winapi/xinput/struct.XINPUT_KEYSTROKE.html) definition, which is missing a `Flags` field present in all versions of XInput's `XINPUT_KEYSTROKE`.)
| ~~`"winapi-0-1"`~~| Interop with [`winapi = "0.1"`](https://docs.rs/winapi/0.1/x86_64-pc-windows-msvc/winapi/) (doesn't compile on modern `rustc`!)
|                   | **Internal features not meant for stable code:** <span style="opacity: 50%">(although they should be safe for `--all-features`)</span>
| `"xxx-docs"`      | Defines what should appear on [docs.rs](https://docs.rs/thindx-xinput/).  Likely to define only the latest `windows-xxx` feature.
