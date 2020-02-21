#[derive(Default)]
pub struct Stone(char, (u64, u64));

impl<'a> Stone {
	pub fn new(color: char, coord: (u64,u64)) -> Self {
		Self(color, coord)
	}

	pub fn display(& mut self) {
		println!("({}, {:?})", &self.0, &self.1);
	}
}
