use crate::{Skey, SkType};

#[derive(Default)]
pub struct ModifierTracker {
	pub shift: bool,
	pub ctrl: bool,
	pub logo: bool,
	pub alt: bool,
}

impl ModifierTracker {
	pub fn update_skey(&mut self, skey: Skey) {
		match skey.ty {
			SkType::Modifier(0) => self.shift = skey.down,
			SkType::Modifier(1) => self.ctrl = skey.down,
			SkType::Modifier(2) => self.logo = skey.down,
			SkType::Modifier(3) => self.alt = skey.down,
			_ => {},
		}
	}
}
