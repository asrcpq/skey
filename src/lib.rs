#[derive(Clone, Copy, Debug)]
pub enum Skey {
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
}

impl Skey {
	pub fn ser(self) -> [u8; 2] {
		match self {
			Self::Ascii(x) => [0, x],
			Self::Common(x) => [1, x],
			Self::Direction(x) => [2, x],
			Self::Func(x) => [3, x],
			Self::System(x) => [4, x],
			Self::Numpad(x) => [5, x],
		}
	}

	pub fn des([b1, b2]: [u8; 2]) -> Option<Self>  {
		let k = match b1 {
			0 => Self::Ascii(b2),
			1 => Self::Common(b2),
			2 => Self::Direction(b2),
			3 => Self::Func(b2),
			4 => Self::System(b2),
			5 => Self::Numpad(b2),
			_ => return None,
		};
		Some(k)
	}
}

#[cfg(feature = "winit")]
pub mod winit;
