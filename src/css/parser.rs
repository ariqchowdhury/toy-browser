use text_parser;
use dom_tree;
use super::stylesheet;

pub struct CssParser {
	pub parse: text_parser::TextParser,
}

impl CssParser {
	pub fn new(parser: text_parser::TextParser) -> CssParser {
		CssParser {
			parse: parser,
		}
	}

	pub fn parse_selector(&mut self) -> Option<stylesheet::Selector> {
		
		match self.parse.peek_char() {
			Some(c) => {
				if c.is_whitespace() {
					self.parse.consume_whitespace();
				} 

				let selector = self.parse.consume_while(|c| c != '{');

				// Get rid of whitespace between text and leading {
				match selector.as_slice().trim_right() {
					"title" => Some(stylesheet::Selector::SelectorType(dom_tree::ElementType::Title)),
					"body" => Some(stylesheet::Selector::SelectorType(dom_tree::ElementType::Body)),
					"h1" | 
					"h2" | 
					"h3" | 
					"h4" => Some(stylesheet::Selector::SelectorType(dom_tree::ElementType::Head)),
					_ => None,
				}
			}
			None => None,
		}

	}

	pub fn parse_declaration(&mut self) -> Option<stylesheet::Declaration> {
		
		match self.parse.peek_char() {
			Some(c) => {
				if c.is_whitespace() {
					self.parse.consume_whitespace();
				} //else if c == 
				None
			}
			None => None,
		}


		/*if self.parse.peek_char() == None {
			return None;
		}

		if self.parse.peek_char().unwrap() == '{' {
			self.parse.consume_char();
			self.parse.consume_whitespace();

			let property = self.parse.consume_while(|c| c != ':');
			self.parse.consume_char();
			self.parse.consume_whitespace();

			let value = self.parse.consume_while(|c| !c.is_whitespace() && c != '}' && c != ';');
			self.parse.consume_whitespace();

			match self.parse.peek_char() {
				Some(c) => None,
				None => None,
			}


		} else {
			None
		}*/
	}
}


	