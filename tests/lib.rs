extern crate ac_browser;

use ac_browser::dom_tree;
use ac_browser::text_parser;
use ac_browser::html_parser;

pub fn test_parse_doctype(doctype: &str, is_proper: bool) {
	let p = text_parser::TextParser::new(doctype.to_string());
	let mut html = html_parser::HtmlParser::new(p);

	match html.parse_doctype() {
		Some(_) => assert!(is_proper),
		None => assert!(!is_proper),
	}
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
fn html_parse_elements() {
	let test_string = 
	"<html>\
		<head>\
			<title>Aliens?\
			</title>\
		</head>\
		<body>\
			A bunch of text that makes up the body\
		</body>
	</html>";
	let p = text_parser::TextParser::new(test_string.to_string());
	let mut html = html_parser::HtmlParser::new(p);
	let mut document = dom_tree::Document::new(dom_tree::Doctype::Html);
	document.element = html.parse_element();

	dom_tree::pretty_print(&mut document);

	assert_eq!(document.element.as_mut().unwrap().e_type, dom_tree::ElementType::ClassE);
	assert_eq!(document.element.as_mut().unwrap().text, None);

	assert!(document.element.as_mut().unwrap().children.len() == 2);

	assert_eq!(document.element.as_mut().unwrap().children[0].e_type, 
			   dom_tree::ElementType::Head);
	assert_eq!(document.element.as_mut().unwrap().children[0].text, None);
	assert!(document.element.as_mut().unwrap().children[0].children.len() == 1);

	assert_eq!(document.element.as_mut().unwrap().children[0].children[0].e_type,
			   dom_tree::ElementType::Title);

	match document.element.as_mut().unwrap().children[0].children[0].text {
		Some(ref s) => if *s != "Aliens?" {assert!(false)},
		None => assert!(false),
	}

	assert_eq!(document.element.as_mut().unwrap().children[1].e_type, 
			   dom_tree::ElementType::Body);

	match document.element.as_mut().unwrap().children[1].text {
		Some(ref s) => if *s != "A bunch of text that makes up the body" {assert!(false)},
		None => assert!(false),
	}

	assert!(document.element.as_mut().unwrap().children[1].children.len() == 0);

}