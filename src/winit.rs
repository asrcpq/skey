use winit::event::VirtualKeyCode as Wk;
use winit::event::KeyboardInput as Wki;
use winit::event::ModifiersState as Wms;

use crate::{Skey, SkType};

pub trait WinitConversion: Sized {
	fn from_wki(wk: Wki) -> Option<Self>;
	fn to_wki(self) -> Option<Wki>;
}

impl WinitConversion for Skey {
	fn from_wki(wki: Wki) -> Option<Skey> {
		let vkc = wki.virtual_keycode?;
		let ty = match vkc {
			Wk::Left => SkType::Direction(0),
			Wk::Up => SkType::Direction(1),
			Wk::Right => SkType::Direction(2),
			Wk::Down => SkType::Direction(2),
			_ => return None
		};
		Some(Skey {
			down: wki.state == winit::event::ElementState::Pressed,
			ty,
		})
	}

	fn to_wki(self) -> Option<Wki> {
		unimplemented!()
	}
}

#[derive(Default)]
pub struct ModifierTracker {
	shift: bool,
	ctrl: bool,
	menu: bool,
	alt: bool,
}

impl ModifierTracker {
	pub fn update_state(&mut self, ms: Wms) -> Vec<Skey> {
		let mut result = Vec::new();
		if self.shift ^ ms.shift() {
			result.push(Skey {
				ty: SkType::Modifier(0),
				down: ms.shift(),
			});
		}
		if self.ctrl ^ ms.ctrl() {
			result.push(Skey {
				ty: SkType::Modifier(1),
				down: ms.ctrl(),
			});
		}
		if self.menu ^ ms.logo() {
			result.push(Skey {
				ty: SkType::Modifier(2),
				down: ms.logo(),
			});
		}
		if self.alt ^ ms.alt() {
			result.push(Skey {
				ty: SkType::Modifier(3),
				down: ms.alt(),
			});
		}
		result
	}
}
