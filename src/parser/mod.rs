extern crate std;

pub struct Parser {
	pub input: String,
	cursor: usize,
}

impl Parser {
	pub fn new(input: String) -> Parser {
		Parser {
			input: input,
			cursor: 0,
		}
	}

	pub fn consume_char(&mut self) -> Option<char> {
		let mut current_char = self.input.chars().skip(self.cursor);
		self.cursor = self.cursor + 1;

		current_char.next()
	}

	pub fn next_char (&mut self) -> Option<char> {
		let mut current_char = self.input.chars().skip(self.cursor);

		current_char.next()
	}
}