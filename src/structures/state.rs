use crate::*;
use bytemuck::{Pod, Zeroable};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_state)\]
/// XINPUT_STATE
///
/// Packet number + [`Gamepad`] state.
#[derive(Clone, Copy, Debug)]
#[derive(Default, Pod, Zeroable)]
#[repr(C)] pub struct State {
    /// State packet number.
    /// The packet number indicates whether there have been any changes in the state of the controller.
    /// If `packet_number` is the same in sequentially returned [`State`] structures, the controller state has not changed.
    pub packet_number: u32,

    /// The gamepad state.
    pub gamepad:        Gamepad,
}

impl AsRef<Self> for State { fn as_ref(&    self) -> &    Self { self } }
impl AsMut<Self> for State { fn as_mut(&mut self) -> &mut Self { self } }

// N.B. these deref to Gamepad, not to XINPUT_STATE
impl core::ops::Deref for State {
    type Target = Gamepad;
    fn deref(&self) -> &Gamepad { &self.gamepad }
}

impl core::ops::DerefMut for State {
    fn deref_mut(&mut self) -> &mut Gamepad { &mut self.gamepad }
}

#[test] fn test_traits_for_coverage() {
    let _state = State::default();
    let _state = State::zeroed();
    let mut _state = _state.clone();
    let _ = _state.left_trigger;
    _state.left_trigger = 42;
    dbg!(_state);
}

//#cpp2rust XINPUT_STATE                = xinput::State
