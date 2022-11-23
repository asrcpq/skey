use crate::{Skey, Sktype};

#[derive(Default, Debug)]
pub struct ModifierTracker {
	pub shift: bool,
	pub ctrl: bool,
	pub logo: bool,
	pub alt: bool,
}

impl ModifierTracker {
	pub fn update_skey(&mut self, skey: Skey) {
		match skey.ty {
			Sktype::Modifier(0) => self.shift = skey.down,
			Sktype::Modifier(1) => self.ctrl = skey.down,
			Sktype::Modifier(2) => self.logo = skey.down,
			Sktype::Modifier(3) => self.alt = skey.down,
			_ => {},
		}
	}
}
