#[derive(Default)]
pub struct Stone(char);

impl<'a> Stone {
	pub fn new(color: char) -> Self {
		Self (color)
	}

	pub fn color(& self) -> char {
		self.0
	}

	pub fn place(& mut self, color: char) {
		self.0 = color;
	}
}
