use rust_stddep::winit::event::VirtualKeyCode as Wk;
use rust_stddep::winit::event::KeyboardInput as Wki;
use rust_stddep::winit::event::ModifiersState as Wms;
use rust_stddep::winit::event::ElementState as Wes;

use crate::{Skey, Sktype};
use crate::modtrack::ModifierTracker;

pub trait WinitConversion: Sized {
	fn from_wki(wk: Wki) -> Option<Self>;
	fn to_wki(self) -> Option<Wki>;
}

impl WinitConversion for Skey {
	fn from_wki(wki: Wki) -> Option<Skey> {
		let vkc = wki.virtual_keycode?;
		let ty = match vkc {
			Wk::Left => Sktype::Direction(0),
			Wk::Up => Sktype::Direction(1),
			Wk::Right => Sktype::Direction(2),
			Wk::Down => Sktype::Direction(3),
			Wk::Insert => Sktype::Direction(4),
			Wk::Delete => Sktype::Direction(5),
			Wk::PageUp => Sktype::Direction(6),
			Wk::PageDown => Sktype::Direction(7),
			Wk::Home => Sktype::Direction(8),
			Wk::End => Sktype::Direction(9),
			Wk::Space => Sktype::Ascii(b' '),
			Wk::A => Sktype::Ascii(b'a'),
			Wk::B => Sktype::Ascii(b'b'),
			Wk::C => Sktype::Ascii(b'c'),
			Wk::D => Sktype::Ascii(b'd'),
			Wk::E => Sktype::Ascii(b'e'),
			Wk::F => Sktype::Ascii(b'f'),
			Wk::G => Sktype::Ascii(b'g'),
			Wk::H => Sktype::Ascii(b'h'),
			Wk::I => Sktype::Ascii(b'i'),
			Wk::J => Sktype::Ascii(b'j'),
			Wk::K => Sktype::Ascii(b'k'),
			Wk::L => Sktype::Ascii(b'l'),
			Wk::M => Sktype::Ascii(b'm'),
			Wk::N => Sktype::Ascii(b'n'),
			Wk::O => Sktype::Ascii(b'o'),
			Wk::P => Sktype::Ascii(b'p'),
			Wk::Q => Sktype::Ascii(b'q'),
			Wk::R => Sktype::Ascii(b'r'),
			Wk::S => Sktype::Ascii(b's'),
			Wk::T => Sktype::Ascii(b't'),
			Wk::U => Sktype::Ascii(b'u'),
			Wk::V => Sktype::Ascii(b'v'),
			Wk::W => Sktype::Ascii(b'w'),
			Wk::X => Sktype::Ascii(b'x'),
			Wk::Y => Sktype::Ascii(b'y'),
			Wk::Z => Sktype::Ascii(b'z'),
			Wk::Key0 => Sktype::Ascii(b'0'),
			Wk::Key1 => Sktype::Ascii(b'1'),
			Wk::Key2 => Sktype::Ascii(b'2'),
			Wk::Key3 => Sktype::Ascii(b'3'),
			Wk::Key4 => Sktype::Ascii(b'4'),
			Wk::Key5 => Sktype::Ascii(b'5'),
			Wk::Key6 => Sktype::Ascii(b'6'),
			Wk::Key7 => Sktype::Ascii(b'7'),
			Wk::Key8 => Sktype::Ascii(b'8'),
			Wk::Key9 => Sktype::Ascii(b'9'),
			Wk::F1 => Sktype::Func(1),
			Wk::F2 => Sktype::Func(2),
			Wk::F3 => Sktype::Func(3),
			Wk::F4 => Sktype::Func(4),
			Wk::F5 => Sktype::Func(5),
			Wk::F6 => Sktype::Func(6),
			Wk::F7 => Sktype::Func(7),
			Wk::F8 => Sktype::Func(8),
			Wk::F9 => Sktype::Func(9),
			Wk::F10 => Sktype::Func(10),
			Wk::F11 => Sktype::Func(11),
			Wk::F12 => Sktype::Func(12),
			Wk::F13 => Sktype::Func(13),
			Wk::F14 => Sktype::Func(14),
			Wk::F15 => Sktype::Func(15),
			Wk::F16 => Sktype::Func(16),
			Wk::F17 => Sktype::Func(17),
			Wk::F18 => Sktype::Func(18),
			Wk::F19 => Sktype::Func(19),
			Wk::F20 => Sktype::Func(20),
			Wk::F21 => Sktype::Func(21),
			Wk::F22 => Sktype::Func(22),
			Wk::F23 => Sktype::Func(23),
			Wk::F24 => Sktype::Func(24),
			_ => return None
		};
		Some(Skey {
			down: wki.state == Wes::Pressed,
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
				ty: Sktype::Modifier(0),
				down: ms.shift(),
			});
		}
		if self.ctrl ^ ms.ctrl() {
			self.ctrl = ms.ctrl();
			result.push(Skey {
				ty: Sktype::Modifier(1),
				down: ms.ctrl(),
			});
		}
		if self.logo ^ ms.logo() {
			self.logo = ms.logo();
			result.push(Skey {
				ty: Sktype::Modifier(2),
				down: ms.logo(),
			});
		}
		if self.alt ^ ms.alt() {
			self.alt = ms.alt();
			result.push(Skey {
				ty: Sktype::Modifier(3),
				down: ms.alt(),
			});
		}
		result
	}
}
