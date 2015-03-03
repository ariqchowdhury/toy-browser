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
	pub fn peek_char(&mut self) -> Option<char> {
		let mut current_char = self.input.chars().skip(self.cursor);

		current_char.next()
	}

	/// Consume characters until condition is false. Increase cursor
	/// by number of chars consumed. 
	/// Return a string of the consumed characters. 
	pub fn consume_while<F: Fn(char) -> bool>(&mut self, cond: F) -> String {
		let mut result = String::new();
		let mut next_char : char;

		loop {
			match self.peek_char() {
				Some(c) => next_char = c,
				None => break,
			}

			if cond(next_char) == false { break; }
			else {
				result.push(self.consume_char().unwrap());				
			}
		}

		result
	}

	/// Consume whitespace characters until non-whitespace char is hit
	pub fn consume_whitespace(&mut self) {
		self.consume_while(CharExt::is_whitespace);
	}
}

