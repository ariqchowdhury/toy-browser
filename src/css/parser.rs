use text_parser;
use dom_tree;
use super::stylesheet;

pub struct CssParser {
	parse: text_parser::TextParser,
}

impl CssParser {
	pub fn new(input: String) -> CssParser {
		let parser = text_parser::TextParser::new(input);
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

	pub fn parse_css(&mut self) -> stylesheet::StyleSheet {
		let mut rule = stylesheet::Rule::new();

		while !self.parse.end_of_string() {
			let sel = self.parse_selector();
			let dec = self.parse_declaration();

			if sel.is_none() {
				continue;
			}

			rule.rule_map.insert(sel.unwrap(), dec);
		}

		stylesheet::StyleSheet {
			ruleset: rule,
		}
	}
}

#[test]
fn test_parse_valid_declaration() {
	let mut css = CssParser::new("{ font-size: bold }".to_string());

	let mut dec = css.parse_declaration();
	assert_eq!(dec.len(), 1);
	assert!(dec.pop().unwrap().property_name == stylesheet::Property::FontSize);
}

#[test]
fn test_parse_invalid_a_declaration() {
	let mut css = CssParser::new("{ : bold }".to_string());

	let dec = css.parse_declaration();
	assert!(dec.is_empty());
}

#[test]
fn test_parse_invalid_b_declaration() {
	let mut css = CssParser::new("{ font-size  }".to_string());

	let dec = css.parse_declaration();
	assert!(dec.is_empty());
}	

#[test]
fn test_parse_invalid_c_declaration() {
	let mut css = CssParser::new("{ font-size ;".to_string());

	let dec = css.parse_declaration();
	assert!(dec.is_empty());
}	

#[test]
fn test_parse_valid_multiline_declaration() {
	let dec_text = "{ font-size: bold; \
					   line-height: 23px; }";
	let mut css = CssParser::new(dec_text.to_string());

	let mut dec = css.parse_declaration();
	assert_eq!(dec.len(), 2);
	assert!(dec.pop().unwrap().property_name == stylesheet::Property::LineHeight);
	assert!(dec.pop().unwrap().property_name == stylesheet::Property::FontSize);

}

#[test]
fn test_parse_full_sel_dec_one_line() {
	let css_text = "h1 { font-size: 12px }";
	let mut css = CssParser::new(css_text.to_string());

	let sel = css.parse_selector();
	let dec = css.parse_declaration();

	assert!(sel.is_some() == true);
	assert_eq!(dec.len(), 1);
}

#[test]
fn test_full_css_parse_one_line() {
	let css_text = "h1 { font-size: 12px }";
	let mut css = CssParser::new(css_text.to_string());	

	let stylesheet = css.parse_css();
	let rules = &stylesheet.ruleset.rule_map;
	let head = &stylesheet::Selector::SelectorType(dom_tree::ElementType::Head);

	assert!(!rules.is_empty());
	assert!(rules
			.contains_key(head));

	let decs = rules.get(head);

	assert!(decs.is_some());
	assert_eq!(decs.unwrap().len(), 1);
	assert!(decs.unwrap()[0].property_name == stylesheet::Property::FontSize);
	assert!(decs.unwrap()[0].property_value == stylesheet::Value::Placeholder);

}

#[test]
fn test_full_css_parse_multi_line() {
	let num_decs = 3;
	let css_text = "h1 {
						font-size: 12px;
						line-height: 32px;
						color: red
					}";
	let mut css = CssParser::new(css_text.to_string());	

	let stylesheet = css.parse_css();
	let rules = &stylesheet.ruleset.rule_map;
	let head = &stylesheet::Selector::SelectorType(dom_tree::ElementType::Head);
	let decs = rules.get(head);

	assert!(!rules.is_empty());
	assert!(rules
			.contains_key(head));

	let props = [stylesheet::Property::FontSize, 
				 stylesheet::Property::LineHeight,
				 stylesheet::Property::Color];

	assert!(decs.is_some());
	assert_eq!(decs.unwrap().len(), num_decs);

	for i in 0..num_decs {
		assert!(decs.unwrap()[i].property_name == props[i]);
		assert!(decs.unwrap()[i].property_value == stylesheet::Value::Placeholder);
	}

}

#[test]
fn test_full_css_parse_multi_selects() {
	let num_decs = 3;

	let css_text = "h1 {
						font-size: 12px;
						line-height: 32px;
						color: red
					}

					body {
						color: red;
						font-size: 32px;
						line-height: 34px	
					}";
	let mut css = CssParser::new(css_text.to_string());	

	let stylesheet = css.parse_css();
	let rules = &stylesheet.ruleset.rule_map;
	let head = &stylesheet::Selector::SelectorType(dom_tree::ElementType::Head);
	let body = &stylesheet::Selector::SelectorType(dom_tree::ElementType::Body);	
	let decs_head = rules.get(head);
	let decs_body = rules.get(body);

	assert!(!rules.is_empty());
	assert!(rules
			.contains_key(head));
	assert!(rules
			.contains_key(body));

	let props = [stylesheet::Property::FontSize, 
				 stylesheet::Property::LineHeight,
				 stylesheet::Property::Color];

	let body_props = [stylesheet::Property::Color, 
				 	  stylesheet::Property::FontSize,
					  stylesheet::Property::LineHeight];

	assert!(decs_head.is_some());
	assert!(decs_body.is_some());
	assert_eq!(decs_head.unwrap().len(), num_decs);
	assert_eq!(decs_body.unwrap().len(), num_decs);

	for i in 0..num_decs {
		assert!(decs_head.unwrap()[i].property_name == props[i]);
		assert!(decs_head.unwrap()[i].property_value == stylesheet::Value::Placeholder);	
	}

	for i in 0..num_decs {
		assert!(decs_body.unwrap()[i].property_name == body_props[i]);
		assert!(decs_body.unwrap()[i].property_value == stylesheet::Value::Placeholder);
	}
}
