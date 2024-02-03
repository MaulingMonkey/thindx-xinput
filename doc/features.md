# Crate Features

| Crate Feature     | Description   |
| ------------------| --------------|
| `"undocumented"`  | Enable undocumented XInput APIs which are exported by ordinal such as: <br> [`get_state_ex`], [`power_off_controller`], ...
|                   | **Features for interop with other crates:** (e.g. [`AsRef`], [`AsMut`], [`From`], [`Into`] for all C++ `struct`s)
| `"winapi-0-3"`    | Interop with [`winapi = "0.3"`](https://docs.rs/winapi/0.3/)
| `"winapi-0-2"`    | Interop with [`winapi = "0.2"`](https://docs.rs/winapi/0.2/x86_64-pc-windows-gnu/winapi/) [⚠️ Interop Errata]
| ~~`"winapi-0-1"`~~| ~~Interop with [`winapi = "0.1"`](https://docs.rs/winapi/0.1/x86_64-pc-windows-msvc/winapi/)~~ <span style="opacity: 50%">(this version of `winapi` doesn't compile on modern rustc)</span>
|
| <code style="text-wrap: nowrap;">\"windows-sys-0-52\"</code> | Interop with [`windows-sys = "0.52"`](https://docs.rs/windows-sys/0.52/)
| `"windows-sys-0-48"`      | Interop with [`windows-sys = "0.48"`](https://docs.rs/windows-sys/0.48/)
| `"windows-sys-0-45"`      | Interop with [`windows-sys = "0.45"`](https://docs.rs/windows-sys/0.45/)
| `"windows-sys-0-42"`      | Interop with [`windows-sys = "0.42"`](https://docs.rs/windows-sys/0.42/) [⚠️ Interop Errata]
| `"windows-sys-0-36"`      | Interop with [`windows-sys = "0.36"`](https://docs.rs/windows-sys/0.36/) [⚠️ Interop Errata]
| `"windows-sys-0-35"`      | Interop with [`windows-sys = "0.35"`](https://docs.rs/windows-sys/0.35/) [⚠️ Interop Errata]
| `"windows-sys-0-34"`      | Interop with [`windows-sys = "0.34"`](https://docs.rs/windows-sys/0.34/) [⚠️ Interop Errata]
| `"windows-sys-0-33"`      | Interop with [`windows-sys = "0.33"`](https://docs.rs/windows-sys/0.33/) [⚠️ Interop Errata]
| `"windows-sys-0-32"`      | Interop with [`windows-sys = "0.32"`](https://docs.rs/windows-sys/0.32/) [⚠️ Interop Errata]
| ~~`"windows-sys-0-31"`~~  | ~~Interop with [`windows-sys = "0.32"`](https://docs.rs/windows-sys/0.31/)~~ <span style="opacity: 50%">(this version of `windows-sys` was yanked)</span>
| `"windows-sys-0-30"`      | Interop with [`windows-sys = "0.30"`](https://docs.rs/windows-sys/0.30/) [⚠️ Interop Errata]
| `"windows-sys-0-29"`      | Interop with [`windows-sys = "0.29"`](https://docs.rs/windows-sys/0.29/) [⚠️ Interop Errata]
| `"windows-sys-0-28"`      | Interop with `windows-sys = "0.28"` [⚠️ Interop Errata] <span style="opacity: 50%">(missing a useful docs.rs)</span>
| `"windows-sys-0-27"`      | Interop with `windows-sys = "0.27"` [⚠️ Interop Errata]
|
|                   | **Internal features not meant for stable code:** <span style="opacity: 50%">(although they should be safe for `--all-features`)</span>
| `"xxx-docs"`      | Defines what should appear on [docs.rs](https://docs.rs/thindx-xinput/).  Likely to define only the latest `windows-xxx` feature.



# Interop Errata

Multiple crates misdefine XInput structures.
While this crate often *could* still implement some [`From`] / [`Into`] conversions (via truncation of mis-sized fields etc.), the use of these broken-ABI types suggests a likelyhood of FFI interop bugs.
To help catch these bugs, interop for these types - when their ABIs are incorrect - is simply not implemented.
Interop for versions of the crates where the ABIs are correct is implemented as usual.

*   [`winapi`](https://docs.rs/winapi/) before `0.3.0` misdefines `XINPUT_KEYSTROKE` as missing a `Flags` field.
*   [`windows-sys`](https://docs.rs/windows-sys/) before `0.45.0` misdefines multiple fields for `XINPUT_BATTERY_INFORMATION` and `XINPUT_CAPABILITIES` as [`u32`]s instead of [`u8`]s.



[⚠️ Interop Errata]:    #interop-errata
