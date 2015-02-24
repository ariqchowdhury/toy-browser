extern crate std;

use std::fmt;

#[derive(Debug, Copy)]
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
	pub element: Element,
}

pub struct Element {
	e_type: ElementType,
	pub text: Box<Option<String>>,
	pub children: Vec<Element>,
}

impl fmt::Display for Element {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self.text {
			Some(ref s) => write!(f, "|__Element: {:?}, Text: {}", self.e_type, *s),
			None    => write!(f, "|__Element: {:?}", self.e_type),
		}
	}
}

impl Document {
	pub fn new (d_type: Doctype) -> Document {
		Document {
			d_type: d_type,
			element: Element::new_root(ElementType::ClassE),
		}
	}
}

pub fn pretty_print(doc: &Document) {
	println!("Document");
	println!("|__Doctype: {:?}", doc.d_type);

	pretty_print_element(0, &doc.element);
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

impl Element {

	fn new_root (e_type: ElementType) -> Element {
		let vec = Vec::new();
		let t : Box<Option<String>> = Box::new(None);
		Element {
			e_type: e_type,
			text: t,
			children: vec,
		}
	}

	pub fn add_child (&mut self, e_type: ElementType, text: Option<String>) {
		let vec = Vec::new();
		let t : Box<Option<String>> = Box::new(text);

		let e = Element {
			e_type: e_type,
			text: t.clone(),
			children: vec,
		};

		self.children.push(e);
	}

}
