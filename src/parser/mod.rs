extern crate std;

/// Parser input : String is the target to be parsed. cursor keeps track of the
/// current position in parsing. Parse functions 'consume' characters by 
/// increasing the cursor. 
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

	/// Return the current character tracked by cursor. Increase cursor by
	/// one position
	pub fn consume_char(&mut self) -> Option<char> {
		let mut current_char = self.input.chars().skip(self.cursor);
		self.cursor = self.cursor + 1;

		current_char.next()
	}

	/// Return the current character tracked by cursor. Cursor remains at
	/// current position
	pub fn peek_char (&mut self) -> Option<char> {
		let mut current_char = self.input.chars().skip(self.cursor);

		current_char.next()
	}
}