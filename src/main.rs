extern crate browser;

use browser::dom_tree;

fn main() {
	let s: Option<String> = Some("ttiittllee".to_string());
	let s2: Option<String> = Some("hheeaadd".to_string());
	let s3: Option<String> = Some("bbooddyy".to_string());

	let ss: Option<String> = Some("ttiittllee".to_string());
	let ss2: Option<String> = Some("hheeaadd".to_string());
	let ss3: Option<String> = Some("bbooddyy".to_string());

	let mut document = dom_tree::Document::new(dom_tree::Doctype::Html);
	document.element.add_child(dom_tree::ElementType::Title, s);
	document.element.add_child(dom_tree::ElementType::Head, s2);
	document.element.add_child(dom_tree::ElementType::Body, s3);

	document.element.children[1].add_child(dom_tree::ElementType::Body, ss);
	document.element.children[1].add_child(dom_tree::ElementType::Body, ss2);
	document.element.children[1].add_child(dom_tree::ElementType::Body, ss3);
	
	dom_tree::pretty_print(&document);
}