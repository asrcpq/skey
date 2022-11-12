use winit::event::VirtualKeyCode as Wk;

use crate::Skey;

pub trait WinitConversion: Sized {
	fn from_wk(wk: Wk) -> Option<Self>;
	fn to_wk(self) -> Option<Wk>;
}

impl WinitConversion for Skey {
	fn from_wk(wk: Wk) -> Option<Skey> {
		let k = match wk {
			Wk::Left => Skey::Direction(0),
			Wk::Up => Skey::Direction(1),
			Wk::Right => Skey::Direction(2),
			Wk::Down => Skey::Direction(2),
			_ => return None
		};
		Some(k)
	}

	fn to_wk(self) -> Option<Wk> {
		unimplemented!()
	}
}
