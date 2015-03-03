use super::parser;
use super::dom_tree;

pub struct HtmlParser {
	pub parse: parser::Parser,	
}

impl HtmlParser {
	pub fn new(parser: parser::Parser) -> HtmlParser {
		HtmlParser {
			parse: parser,
		}
	}

	/// html document begins with "<!DOCTYPE html>"; parse this and
	/// return a Document object if it parses correctly
	pub fn parse_doctype(&mut self) -> Option<dom_tree::Document> {
		let mut doctype_header = "".to_string();
		let mut doctype_type = "".to_string();
		let mut result: Option<dom_tree::Document>;

		if self.parse.consume_if_char_matches('<') == true {
			if self.parse.consume_if_char_matches('!') == true {
				doctype_header = self.parse.consume_while(|c| match c {
					'a'...'z' | 'A'...'Z' => true,
					_ => false
				});

				self.parse.consume_whitespace();

				doctype_type = self.parse.consume_while(|c| match c {
					'a'...'z' | 'A'...'Z' => true,
					_ => false
				});

				self.parse.consume_whitespace();
			}
		} 

		if self.parse.consume_if_char_matches('>') == true &&
		   doctype_header == "DOCTYPE" &&
		   doctype_type == "html" {

		   result = Some(dom_tree::Document::new(dom_tree::Doctype::Html));

		} else {
			result = None;
		}

		result
	}

	pub fn parse_element(&mut self) /*-> dom_tree::Element*/ {

	}
}
