extern crate std;

use std::fmt;

#[derive(Debug, Copy, PartialEq)]
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