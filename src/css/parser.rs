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
		
		let mut prop : Option<String> = None;
		let mut val : Option<String> = None;

		let mut retval : Option<stylesheet::Declaration> = None;

		loop {
			if self.parse.end_of_string() {
				break;
			}
			match self.parse.peek_char() {
				Some(c) => {

					match c {
						c if c.is_whitespace() => {
							self.parse.consume_whitespace();
						}
						'{' => {
							self.parse.consume_char();
						}
						':' => {
							self.parse.consume_char();
							val = Some(self.parse.consume_while(|c| match c {
								';' | '}' => false,
								_ => true}));
						}
						';' | '}' => { 
							let line_break = self.parse.consume_char();
							
							if prop.is_some() && val.is_some() {
								let parsed_prop = stylesheet::string_to_property(&prop.unwrap());
								let parsed_val = stylesheet::string_to_value(&val.unwrap());

								if parsed_prop.is_some() && parsed_val.is_some() {
									retval = Some(stylesheet::Declaration { 
										property_name: parsed_prop.unwrap(), 
										property_value: parsed_val.unwrap()
									});	
								} else {
									retval = None;
								}
								
							}
							break;
						}
						_ => {
							prop = Some(self.parse.consume_while(|c| match c {
								':' => false,
								_ => true}));

						}
					}
				} // Some
				None => {
					retval = None;
				}
			} // match
		} //loop
		retval
	}
}


	