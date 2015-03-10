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

	pub fn peek_next_char(&mut self) -> Option<char> {
		let mut current_char = self.input.chars().skip(self.cursor + 1);

		current_char.next()
	}

	/// Return true if the String is consumed
	pub fn end_of_string(&mut self) -> bool {
		self.cursor >= self.input.len()
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

	pub fn consume_until_whitespace(&mut self) -> String {
		self.consume_while(|c| !c.is_whitespace())
	}

	/// Consume if given test_char is the current char
	/// Return true if char was consumed, else false
	pub fn consume_if_char_matches(&mut self, test: char) -> bool {
		let mut result : bool;

		match self.peek_char() {
			Some(c) => {
				if c == test {
					self.consume_char();
					result = true;
				} else {
					result = false;
				}
			}
			None => {result = false;}
		}

		result
	}
}

#[cfg(test)]
mod test_parser {
		#[test]
	fn parser_consume_char_if_match() {
		let test_string = "FB";
		let mut p = super::Parser::new(test_string.to_string());

		assert!(!p.consume_if_char_matches('f'));
		assert!(p.consume_if_char_matches('F'));
		assert!(!p.consume_if_char_matches('f'));
		assert!(p.consume_if_char_matches('B')); 
	}

	#[test]
	fn parser_consume_char() {
		let test_string = "TeStInG sTrInG";
		let mut p = super::Parser::new(test_string.to_string());

		for i in 0..14 {
			match p.consume_char() {
				Some(c) => assert_eq!(test_string.char_at(i), c),
				None => println!("Done"),
			}
		}	

	}

	#[test]
	fn parser_next_char() {
		let test_string = "TeStInG sTrInG";
		let mut p = super::Parser::new(test_string.to_string());

		for _ in 0..14 {
			match p.peek_char() {
				Some(c) => assert_eq!(test_string.char_at(0), c),
				None => println!("Done"),
			}
		}	

		for _ in 0..14 {
			match p.peek_next_char() {
				Some(c) => assert_eq!(test_string.char_at(1), c),
				None => println!("Done"),
			}
		}
	}

	#[test]
	fn parser_end_of_string() {
		let test_string = "1234567";
		let mut s = String::new();
		let mut p = super::Parser::new(test_string.to_string());

		for _ in 0..7 {
			s.push(p.consume_char().unwrap());
		}

		assert!(s == test_string);
		assert!(p.end_of_string());
	}

	#[test]
	fn parser_consume_whitespace() {
		let test_string = "     F";
		let mut p = super::Parser::new(test_string.to_string());

		p.consume_whitespace();

		match p.peek_char() {
			Some(c) => assert_eq!('F', c),
			None => println!("Done"),
		}
	}

	#[test]
	fn parser_consume_while() {
		let test_string = "stuff in front of <html>";
		let mut p = super::Parser::new(test_string.to_string());

		p.consume_while(|c| c != '<');

		match p.peek_char() {
			Some(c) => assert_eq!('<', c),
			None => println!("Done"),
		}
	}
}