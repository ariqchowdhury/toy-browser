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
		let mut doctype_header = String::new();
		let mut doctype_type = String::new();
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

	fn parse_node(&mut self) -> String {
		self.parse.consume_while(|c| c != '<');
		self.parse.consume_char();

		// consume first word in tag; breaks at whitespace in case there is a class
		// tag, or just goes to end of node
		let ele_type = self.parse.consume_while(|c| !c.is_whitespace() && c != '>');

		// Consume to end of node in case there were class tags.
		self.parse.consume_while(|c| c != '>');
		self.parse.consume_char();

		ele_type
	}

	fn parse_dom_text(&mut self) -> String {
		self.parse.consume_while(|c| c != '<')
	}

	fn give_element_type(string :&str) -> Option<dom_tree::ElementType> {
		match string {
			"html" => Some(dom_tree::ElementType::ClassE),
			"head" => Some(dom_tree::ElementType::Head), 
			"title" => Some(dom_tree::ElementType::Title), 
			"body" => Some(dom_tree::ElementType::Body),
			_ => {println!("failed match");return None},
		}
	}

	pub fn parse_element(&mut self) -> Option<dom_tree::Element> {
		
		// parse a node
		// < to >
		let ele_type = self.parse_node();

		// if the next thing is not '<', parse until '<'. that is your text
		let dom_text = self.parse_dom_text();
		let t = if dom_text == "" { None } else { Some(dom_text) };
		
		// create an element
		let new_ele = HtmlParser::give_element_type(ele_type.as_slice());

		let mut element : Option<dom_tree::Element>;
		match new_ele {
			Some(s) => element = Some(dom_tree::new_element(s, t)),
			None => element = None,
		}

		if element.is_none() {
			return None;
		}

		loop {
			if self.parse.end_of_string() {
				break;
			}

			// tag candidate is either closing a tag or not. 
			// if its closing ('</ >'), then consume it and break
			// if its not, it must be another node that needs to be
			// added
			if self.parse.peek_next_char().unwrap() == '/' {
				self.parse.consume_while(|c| c != '>');
				self.parse.consume_char();
				break;
			} else {
				let next_node = self.parse_element();
				if next_node.is_some() {
					element.as_mut().unwrap().add_child_element(next_node.unwrap());
				}
			}
		}

		element
	}
}
