extern crate std;

use std::fmt;

#[derive(Debug, Copy, PartialEq, Hash, Eq, Clone)]
pub enum ElementType {
	ClassE,
	Head,
	Title,
	Body,
}

#[derive(Debug, Copy)]
pub enum Doctype {
	Html,
}

pub struct Document {
	d_type: Doctype,
	pub element: Option<Element>,
}

/// Elements are nodes in the DOM tree
#[derive(Clone)]
pub struct Element {
	pub e_type: ElementType,
	pub text: Option<String>,
	pub children: Vec<Element>,
}

/// Implement to help pretty_print display the DOM tree structure
impl fmt::Display for Element {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.text {
			Some(ref s) => write!(f, "|__Element: {:?}, Text: {}", self.e_type, *s),
			None    => write!(f, "|__Element: {:?}", self.e_type),
		}
	}
}

impl Document {
	pub fn new(d_type: Doctype) -> Document {
		Document {
			d_type: d_type,
			element: None,
		}
	}
}
	
pub fn new_element(e_type: ElementType, text: Option<String>) -> Element {
	let vec = Vec::new();

	Element {
		e_type: e_type,
		text: text,
		children: vec,
	}
}

impl Element {

	/// Used to create the first Element in the DOM-tree. Called
	/// when a new Document is created
	pub fn new_root(e_type: ElementType) -> Element {
		let vec = Vec::new();

		Element {
			e_type: e_type,
			text: None,
			children: vec,
		}
	}

	/// Add a child element to an element. 'text' is optional. 
	pub fn add_child(&mut self, e_type: ElementType, text: Option<String>) {
		let vec = Vec::new();

		let e = Element {
			e_type: e_type,
			text: text,
			children: vec,
		};

		self.children.push(e);
	}

	pub fn add_child_element(&mut self, e: Element) {
		self.children.push(e);
	}

}

/// Print the DOM-tree of the given 'doc' in a readable way
pub fn pretty_print(doc: &mut Document) {
	println!("Document");
	println!("|__Doctype: {:?}", doc.d_type);

	pretty_print_element(0, &doc.element.as_mut().unwrap());
}

fn pretty_print_element(depth: i32, e: &Element) {
	let mut s = "".to_string();
	for _ in 0..depth {
		s.push(' ');
		s.push(' ');
		s.push(' ');
		s.push(' ');
	}
	println!("{}{}", s, e);

	for c in e.children.iter() {
		pretty_print_element(depth + 1, c);
	}
}

#[cfg(test)]
mod test_dom_tree {

	#[test]
	fn dom_add_child() {
		let s: Option<String> = Some("x".to_string());
		let s2: Option<String> = Some("y".to_string());
		let s3: Option<String> = Some("z".to_string());

		let ss: Option<String> = Some("a".to_string());
		let ss2: Option<String> = Some("b".to_string());
		let ss3: Option<String> = Some("c".to_string());

		let mut document = super::Document::new(super::Doctype::Html);
		document.element = Some(super::Element::new_root(super::ElementType::ClassE));
		document.element.as_mut().unwrap().add_child(super::ElementType::Title, s);
		document.element.as_mut().unwrap().add_child(super::ElementType::Head, s2);
		document.element.as_mut().unwrap().add_child(super::ElementType::Body, s3);

		document.element.as_mut().unwrap().children[1].add_child(super::ElementType::Body, ss);
		document.element.as_mut().unwrap().children[1].add_child(super::ElementType::Body, ss2);
		document.element.as_mut().unwrap().children[1].add_child(super::ElementType::Body, ss3);

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