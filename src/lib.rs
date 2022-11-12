#[derive(Clone, Copy, Debug)]
pub enum SkType {
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
	// shift, ctrl, menu, alt
	// atw winit cannot distinguish left and right
	Modifier(u8),
}

#[derive(Clone, Copy, Debug)]
pub struct Skey {
	pub down: bool,
	pub ty: SkType,
}

impl Skey {
	pub fn ser(self) -> [u8; 3] {
		let d = self.down as u8;
		match self.ty {
			SkType::Ascii(x) => [d, 0, x],
			SkType::Common(x) => [d, 1, x],
			SkType::Direction(x) => [d, 2, x],
			SkType::Func(x) => [d, 3, x],
			SkType::System(x) => [d, 4, x],
			SkType::Numpad(x) => [d, 5, x],
			SkType::Modifier(x) => [d, 6, x],
		}
	}

	pub fn des([down, b1, b2]: [u8; 3]) -> Option<Self>  {
		let ty = match b1 {
			0 => SkType::Ascii(b2),
			1 => SkType::Common(b2),
			2 => SkType::Direction(b2),
			3 => SkType::Func(b2),
			4 => SkType::System(b2),
			5 => SkType::Numpad(b2),
			6 => SkType::Modifier(b2),
			_ => return None,
		};
		Some(Self {
			ty,
			down: down != 0,
		})
	}
}

#[cfg(feature = "winit")]
pub mod winit;
