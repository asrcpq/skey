use winit::event::VirtualKeyCode as Wk;
use winit::event::KeyboardInput as Wki;
use winit::event::ModifiersState as Wms;

use crate::{Skey, SkType};
use crate::modtrack::ModifierTracker;

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
			Wk::Down => SkType::Direction(3),
			Wk::Insert => SkType::Direction(4),
			Wk::Delete => SkType::Direction(5),
			Wk::PageUp => SkType::Direction(6),
			Wk::PageDown => SkType::Direction(7),
			Wk::Home => SkType::Direction(8),
			Wk::End => SkType::Direction(9),
			Wk::Space => SkType::Ascii(b' '),
			Wk::A => SkType::Ascii(b'A'),
			Wk::B => SkType::Ascii(b'B'),
			Wk::C => SkType::Ascii(b'C'),
			Wk::D => SkType::Ascii(b'D'),
			Wk::E => SkType::Ascii(b'E'),
			Wk::F => SkType::Ascii(b'F'),
			Wk::G => SkType::Ascii(b'G'),
			Wk::H => SkType::Ascii(b'H'),
			Wk::I => SkType::Ascii(b'I'),
			Wk::J => SkType::Ascii(b'J'),
			Wk::K => SkType::Ascii(b'K'),
			Wk::L => SkType::Ascii(b'L'),
			Wk::M => SkType::Ascii(b'M'),
			Wk::N => SkType::Ascii(b'N'),
			Wk::O => SkType::Ascii(b'O'),
			Wk::P => SkType::Ascii(b'P'),
			Wk::Q => SkType::Ascii(b'Q'),
			Wk::R => SkType::Ascii(b'R'),
			Wk::S => SkType::Ascii(b'S'),
			Wk::T => SkType::Ascii(b'T'),
			Wk::U => SkType::Ascii(b'U'),
			Wk::V => SkType::Ascii(b'V'),
			Wk::W => SkType::Ascii(b'W'),
			Wk::X => SkType::Ascii(b'X'),
			Wk::Y => SkType::Ascii(b'Y'),
			Wk::Z => SkType::Ascii(b'Z'),
			Wk::Key0 => SkType::Ascii(b'0'),
			Wk::Key1 => SkType::Ascii(b'1'),
			Wk::Key2 => SkType::Ascii(b'2'),
			Wk::Key3 => SkType::Ascii(b'3'),
			Wk::Key4 => SkType::Ascii(b'4'),
			Wk::Key5 => SkType::Ascii(b'5'),
			Wk::Key6 => SkType::Ascii(b'6'),
			Wk::Key7 => SkType::Ascii(b'7'),
			Wk::Key8 => SkType::Ascii(b'8'),
			Wk::Key9 => SkType::Ascii(b'9'),
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

pub trait WinitModifier {
	fn update_state(&mut self, ms: Wms) -> Vec<Skey>;
}

impl WinitModifier for ModifierTracker {
	fn update_state(&mut self, ms: Wms) -> Vec<Skey> {
		let mut result = Vec::new();
		if self.shift ^ ms.shift() {
			self.shift = ms.shift();
			result.push(Skey {
				ty: SkType::Modifier(0),
				down: ms.shift(),
			});
		}
		if self.ctrl ^ ms.ctrl() {
			self.ctrl = ms.ctrl();
			result.push(Skey {
				ty: SkType::Modifier(1),
				down: ms.ctrl(),
			});
		}
		if self.logo ^ ms.logo() {
			self.logo = ms.logo();
			result.push(Skey {
				ty: SkType::Modifier(2),
				down: ms.logo(),
			});
		}
		if self.alt ^ ms.alt() {
			self.alt = ms.alt();
			result.push(Skey {
				ty: SkType::Modifier(3),
				down: ms.alt(),
			});
		}
		result
	}
}
