use text_parser;
use dom_tree;
use super::stylesheet;

//use std::vec;

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

	fn add_declaration_if_valid(&mut self, 
								dec_list: &mut Vec<stylesheet::Declaration>,
								prop : &mut Option<String>,
								val : &mut Option<String>) {
		if prop.is_some() && val.is_some() {
			let parsed_prop = stylesheet::string_to_property(&prop.clone().unwrap());
			let parsed_val = stylesheet::string_to_value(&val.clone().unwrap());

			if parsed_prop.is_some() && parsed_val.is_some() {
				dec_list.push(stylesheet::Declaration { 
					property_name: parsed_prop.unwrap(), 
					property_value: parsed_val.unwrap()
				});	
			} 								
		}
	}

	pub fn parse_declaration(&mut self) -> Vec<stylesheet::Declaration> {
		
		let mut prop : Option<String> = None;	
		let mut val : Option<String> = None;

		let mut retval : Vec<stylesheet::Declaration> = Vec::new();

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
						'}' => { 
							self.parse.consume_char();
							self.add_declaration_if_valid(&mut retval, &mut prop, &mut val);
							break;
						}
						';' => {
							self.parse.consume_char();
							self.add_declaration_if_valid(&mut retval, &mut prop, &mut val);
							prop = None;
							val = None;
						}
						_ => {
							prop = Some(self.parse.consume_while(|c| match c {
								':' => false,
								_ => true}));
						}
					}
				} // Some
				None => {}
			} // match
		} //loop
		retval
	}
}

#[test]
fn test_parse_valid_declaration() {
	let dec_text = "{ font-size: bold }";
	let p = text_parser::TextParser::new(dec_text.to_string());
	let mut css = CssParser::new(p);

	let mut dec = css.parse_declaration();
	assert_eq!(dec.len(), 1);
	assert!(dec.pop().unwrap().property_name == stylesheet::Property::FontSize);
}

#[test]
fn test_parse_invalid_a_declaration() {
	let dec_text = "{ : bold }";
	let p = text_parser::TextParser::new(dec_text.to_string());
	let mut css = CssParser::new(p);

	let dec = css.parse_declaration();
	assert!(dec.is_empty());
}

#[test]
fn test_parse_invalid_b_declaration() {
	let dec_text = "{ font-size  }";
	let p = text_parser::TextParser::new(dec_text.to_string());
	let mut css = CssParser::new(p);

	let dec = css.parse_declaration();
	assert!(dec.is_empty());
}	

#[test]
fn test_parse_invalid_c_declaration() {
	let dec_text = "{ font-size ;";
	let p = text_parser::TextParser::new(dec_text.to_string());
	let mut css = CssParser::new(p);

	let dec = css.parse_declaration();
	assert!(dec.is_empty());
}	

#[test]
fn test_parse_valid_multiline_declaration() {
	let dec_text = "{ font-size: bold; \
					   line-height: 23px; }";
	let p = text_parser::TextParser::new(dec_text.to_string());
	let mut css = CssParser::new(p);

	let mut dec = css.parse_declaration();
	assert_eq!(dec.len(), 2);
	assert!(dec.pop().unwrap().property_name == stylesheet::Property::LineHeight);
	assert!(dec.pop().unwrap().property_name == stylesheet::Property::FontSize);

}