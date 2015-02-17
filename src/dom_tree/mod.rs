pub enum ElementType {
	ClassE,
	Head,
	Title,
	Body,
}

pub enum Doctype {
	Html,
}

pub struct Document {
	d_type: Doctype,
	element: Element,
}

pub struct Element {
	e_type: ElementType,
	text: Option<String>,
	children: Vec<Element>,
}

impl Document {
	pub fn new (d_type: Doctype) -> Document {
		Document {
			d_type: d_type,
			element: Element::new_root(ElementType::ClassE),
		}
	}
}

impl Element {

	fn new_root (e_type: ElementType) -> Element {
		let mut vec = Vec::new();
		Element {
			e_type: e_type,
			text: None,
			children: vec,
		}
	}

	pub fn add_child (&mut self, e_type: ElementType, text: Option<String>) {
		let mut vec = Vec::new();

		let e = Element {
			e_type: e_type,
			text: text,
			children: vec,
		};

		self.children.push(e);
	}

}
