#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sktype {
	// printable ascii(including space)
	Ascii(u8),
	// esc, tab, bs, enter, menu
	Common(u8),
	// Left Up Right Down
	// insert delete
	// page up, page down
	// home end
	Direction(u8),
	Func(u8),
	// SysRq, ScrLck, Pause
	System(u8),
	// 0-9 dot
	// + - * /
	// numlck, enter
	Numpad(u8),
	// shift, ctrl, logo, alt
	// atw winit cannot distinguish left and right
	Modifier(u8),
}

#[derive(Clone, Copy, Debug)]
pub struct Skey {
	pub down: bool,
	pub ty: Sktype,
}

impl Skey {
	pub fn ser(self) -> [u8; 3] {
		let d = self.down as u8;
		match self.ty {
			Sktype::Ascii(x) => [d, 0, x],
			Sktype::Common(x) => [d, 1, x],
			Sktype::Direction(x) => [d, 2, x],
			Sktype::Func(x) => [d, 3, x],
			Sktype::System(x) => [d, 4, x],
			Sktype::Numpad(x) => [d, 5, x],
			Sktype::Modifier(x) => [d, 6, x],
		}
	}

	pub fn des([down, b1, b2]: [u8; 3]) -> Option<Self>  {
		let ty = match b1 {
			0 => Sktype::Ascii(b2),
			1 => Sktype::Common(b2),
			2 => Sktype::Direction(b2),
			3 => Sktype::Func(b2),
			4 => Sktype::System(b2),
			5 => Sktype::Numpad(b2),
			6 => Sktype::Modifier(b2),
			_ => return None,
		};
		Some(Self {
			ty,
			down: down != 0,
		})
	}
}

pub mod modtrack;
#[cfg(feature = "winit")]
pub mod winit;
