#![feature(core)]

pub fn test_parse_doctype(doctype: &str, is_proper: bool) {
	let p = parser::Parser::new(doctype.to_string());
	let mut html = html_parser::HtmlParser::new(p);

	match html.parse_doctype() {
		Some(_) => assert!(is_proper),
		None => assert!(!is_proper),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn html_parse_elements() {

	}

	#[test]
	fn html_parse_doctype() {
		let proper_doctype = "<!DOCTYPE html>";
		test_parse_doctype(proper_doctype, true);
	}

	#[test]
	fn html_parse_invalid_doctype() {
		let improper_doctype = "<!DsdfCTYPE html>";
		test_parse_doctype(improper_doctype, false);
	}

	#[test]
	fn parser_consume_char_if_match() {
		let test_string = "FB";
		let mut p = parser::Parser::new(test_string.to_string());

		assert!(!p.consume_if_char_matches('f'));
		assert!(p.consume_if_char_matches('F'));
		assert!(!p.consume_if_char_matches('f'));
		assert!(p.consume_if_char_matches('B')); 
	}

	#[test]
	fn parser_consume_char() {
		let test_string = "TeStInG sTrInG";
		let mut p = parser::Parser::new(test_string.to_string());

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
		let mut p = parser::Parser::new(test_string.to_string());

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
		let mut p = parser::Parser::new(test_string.to_string());

		for _ in 0..7 {
			s.push(p.consume_char().unwrap());
		}

		assert!(s == test_string);
		assert!(p.end_of_string());
	}

	#[test]
	fn parser_consume_whitespace() {
		let test_string = "     F";
		let mut p = parser::Parser::new(test_string.to_string());

		p.consume_whitespace();

		match p.peek_char() {
			Some(c) => assert_eq!('F', c),
			None => println!("Done"),
		}
	}

	#[test]
	fn parser_consume_while() {
		let test_string = "stuff in front of <html>";
		let mut p = parser::Parser::new(test_string.to_string());

		p.consume_while(|c| c != '<');

		match p.peek_char() {
			Some(c) => assert_eq!('<', c),
			None => println!("Done"),
		}
	}

	#[test]
	fn dom_add_child() {
		let s: Option<String> = Some("x".to_string());
		let s2: Option<String> = Some("y".to_string());
		let s3: Option<String> = Some("z".to_string());

		let ss: Option<String> = Some("a".to_string());
		let ss2: Option<String> = Some("b".to_string());
		let ss3: Option<String> = Some("c".to_string());

		let mut document = dom_tree::Document::new(dom_tree::Doctype::Html);
		document.element = Some(dom_tree::Element::new_root(dom_tree::ElementType::ClassE));
		document.element.as_mut().unwrap().add_child(dom_tree::ElementType::Title, s);
		document.element.as_mut().unwrap().add_child(dom_tree::ElementType::Head, s2);
		document.element.as_mut().unwrap().add_child(dom_tree::ElementType::Body, s3);

		document.element.as_mut().unwrap().children[1].add_child(dom_tree::ElementType::Body, ss);
		document.element.as_mut().unwrap().children[1].add_child(dom_tree::ElementType::Body, ss2);
		document.element.as_mut().unwrap().children[1].add_child(dom_tree::ElementType::Body, ss3);

		assert!(document.element.as_mut().unwrap().children.len() == 3);

		assert!(document.element.as_mut().unwrap().children[0].children.len() == 0);
		assert!(document.element.as_mut().unwrap().children[1].children.len() == 3);
		assert!(document.element.as_mut().unwrap().children[2].children.len() == 0);

		assert!(document.element.as_mut().unwrap().children[0].text.is_some());
		assert!(document.element.as_mut().unwrap().children[1].text.is_some());
		assert!(document.element.as_mut().unwrap().children[2].text.is_some());

		assert!(document.element.as_mut().unwrap().children[1].children[0].text.is_some());
		assert!(document.element.as_mut().unwrap().children[1].children[1].text.is_some());
		assert!(document.element.as_mut().unwrap().children[1].children[2].text.is_some());

		match document.element.as_mut().unwrap().children[0].text {
			Some(ref s) => if *s != "x" {assert!(false)},
			None => assert!(false),
		}

		match document.element.as_mut().unwrap().children[1].text {
			Some(ref s) => if *s != "y" {assert!(false)},
			None => assert!(false),
		}

		match document.element.as_mut().unwrap().children[2].text {
			Some(ref s) => if *s != "z" {assert!(false)},
			None => assert!(false),
		}

		match document.element.as_mut().unwrap().children[1].children[0].text {
			Some(ref s) => if *s != "a" {assert!(false)},
			None => assert!(false),
		}

		match document.element.as_mut().unwrap().children[1].children[1].text {
			Some(ref s) => if *s != "b" {assert!(false)},
			None => assert!(false),
		}

		match document.element.as_mut().unwrap().children[1].children[2].text {
			Some(ref s) => if *s != "c" {assert!(false)},
			None => assert!(false),
		}
	}

		
}

pub mod dom_tree;
pub mod parser;
pub mod html_parser;
