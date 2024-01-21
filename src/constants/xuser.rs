//! \[~~microsoft.com~~\]
//! XUSER_\*

#[cfg(doc)] use crate::*;

/// \[~~microsoft.com~~\]
/// XUSER_INDEX_ANY                                                                                                 <br>
/// Pass to [`get_keystroke`] to get [`Keystroke`]s for any/all connected gamepads rather than a specific gamepad.  <br>
/// (The gamepad which send the keystroke can be determined with [`Keystroke::user_index`].)                        <br>
///                                                                                                                 <br>
pub const INDEX_ANY : u32 = 0xFF;

/// \[~~microsoft.com~~\]
/// XUSER_MAX_COUNT (4)
/// &mdash;
/// The maximum number of unique gamepads/users supported by XInput.
/// <br>
/// All user indicies will be less than this (excluding the magic value, [`xuser::INDEX_ANY`].)
pub const MAX_COUNT : u32 = 4;



/// Iterator over valid user indicies (<code>0 .. [xuser::MAX_COUNT]</code>)
pub fn iter() -> impl Iterator<Item = u32> { (0 .. MAX_COUNT).into_iter() }

/// Iterator over a wide range of invalid user indicies for testing purpouses.  Does **not** include [`xuser::INDEX_ANY`].
/// Includes *every* user index from 4 ..= 254, 256, 512, ..., 0x8000_0000
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn invalids() -> impl Iterator<Item = u32> {
    (4 ..= 254).chain((8 .. 32).map(|pow| 1<<pow))
}


//#cpp2rust XUSER_INDEX_ANY     = xinput::xuser::INDEX_ANY
//#cpp2rust XUSER_MAX_COUNT     = xinput::xuser::MAX_COUNT
