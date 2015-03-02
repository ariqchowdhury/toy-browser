extern crate std;

pub struct Parser {
	pub input: String,
	pub cursor: usize,
}

impl Parser {

	pub fn consume_char(&mut self) -> Option<char> {

		let mut current_char = self.input.chars().skip(self.cursor);
		self.cursor = self.cursor + 1;

		current_char.next()
	}
}