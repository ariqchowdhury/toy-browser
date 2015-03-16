extern crate ac_browser;

use ac_browser::dom_tree;
use ac_browser::text_parser;
use ac_browser::html_parser;
use ac_browser::css;
use ac_browser::style_tree;

pub fn test_parse_doctype(doctype: &str, is_proper: bool) {
	let mut html = html_parser::HtmlParser::new(doctype.to_string());

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
	let mut html = html_parser::HtmlParser::new(test_string.to_string());
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

#[test]
fn test_build_style_tree() {
	let html_string = 
	"<html>\
		<head>\
			<title>Aliens?\
			</title>\
		</head>\
		<body>\
			A bunch of text that makes up the body\
		</body>
	</html>";
	let mut html = html_parser::HtmlParser::new(html_string.to_string());
	let mut document = dom_tree::Document::new(dom_tree::Doctype::Html);
	document.element = html.parse_element();

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
	let mut css = css::parser::CssParser::new(css_text.to_string());	
	let stylesheet = css.parse_css();

	let mut style = style_tree::StyleNode::new(&document.element.as_mut().unwrap().children[0]);

	let decs = style.create_list_of_matching_declarations(&stylesheet);

	assert!(decs.is_some());
	assert!(decs.unwrap()[0].property_name == css::stylesheet::Property::FontSize);
	assert!(decs.unwrap()[1].property_name == css::stylesheet::Property::LineHeight);
	assert!(decs.unwrap()[2].property_name == css::stylesheet::Property::Color);
}

fn css_parse_selector(selector_text: &str, should_match: bool) {

	let mut css = css::parser::CssParser::new(selector_text.to_string());

	let selector = css.parse_selector();

	assert!(selector.is_some() == should_match);

}

#[test]
fn parse_title_selector() {
	css_parse_selector("title", true);
	css_parse_selector("body", true);
	css_parse_selector("h1", true);
	css_parse_selector("h2", true);
	css_parse_selector("h3", true);
	css_parse_selector("h4", true);

	css_parse_selector("none", false);

	css_parse_selector("title {", true);
}
