# Alternative Crates

| Comparison            | [`thindx-xinput`](https://docs.rs/thindx-xinput/)                     | [`rusty-xinput`](https://docs.rs/rusty-xinput/latest/rusty_xinput/)   |
| ----------------------| ----------------------------------------------------------------------| ----------------------------------------------------------------------|
| Author                | ✔️ [MaulingMonkey](https://github.com/MaulingMonkey)                  | ✔️ [Lokathor](https://github.com/Lokathor)
| API Design            | ✔️ *Mostly* rustified 1:1 with original API                           | ⚠️ Inconsistently rustified, and a bit cluttered
| Debug                 | ✔️ Overengineered `*.natvis` files                                    | ⚠️ Reasonably straightforward
| DLL Loading           | ⚠️ Implicit and lazy with erroring fallbacks                          | ✔️ Explicit, manual
| Semver                | ❌ Not yet stable                                                     | ✔️ `1.3`
| Soundness             | ✔️ *Thorough* unit testing                                            | ✔️ The APIs are generally safe
| [`winapi`] interop    | ✔️ Spammed `impl From<...>`                                           | ⚠️ Varies (sometimes exposed as `.0`, `.raw`, ...)

[`winapi`]:         https://docs.rs/winapi/0.3/
