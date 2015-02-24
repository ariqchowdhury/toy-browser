#![feature(core)]

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_add_child() {
		let s: Option<String> = Some("x".to_string());
		let s2: Option<String> = Some("y".to_string());
		let s3: Option<String> = Some("z".to_string());

		let ss: Option<String> = Some("a".to_string());
		let ss2: Option<String> = Some("b".to_string());
		let ss3: Option<String> = Some("c".to_string());

		let mut document = dom_tree::Document::new(dom_tree::Doctype::Html);
		document.element.add_child(dom_tree::ElementType::Title, s);
		document.element.add_child(dom_tree::ElementType::Head, s2);
		document.element.add_child(dom_tree::ElementType::Body, s3);

		document.element.children[1].add_child(dom_tree::ElementType::Body, ss);
		document.element.children[1].add_child(dom_tree::ElementType::Body, ss2);
		document.element.children[1].add_child(dom_tree::ElementType::Body, ss3);

		assert!(document.element.children.len() == 3);

		assert!(document.element.children[0].children.len() == 0);
		assert!(document.element.children[1].children.len() == 3);
		assert!(document.element.children[2].children.len() == 0);

		assert!(document.element.children[0].text.is_some());
		assert!(document.element.children[1].text.is_some());
		assert!(document.element.children[2].text.is_some());

		assert!(document.element.children[1].children[0].text.is_some());
		assert!(document.element.children[1].children[1].text.is_some());
		assert!(document.element.children[1].children[2].text.is_some());

		match *document.element.children[0].text {
			Some(ref s) => if *s != "x" {assert!(false)},
			None => assert!(false),
		}

		match *document.element.children[1].text {
			Some(ref s) => if *s != "y" {assert!(false)},
			None => assert!(false),
		}

		match *document.element.children[2].text {
			Some(ref s) => if *s != "z" {assert!(false)},
			None => assert!(false),
		}

		match *document.element.children[1].children[0].text {
			Some(ref s) => if *s != "a" {assert!(false)},
			None => assert!(false),
		}

		match *document.element.children[1].children[1].text {
			Some(ref s) => if *s != "b" {assert!(false)},
			None => assert!(false),
		}

		match *document.element.children[1].children[2].text {
			Some(ref s) => if *s != "c" {assert!(false)},
			None => assert!(false),
		}
	}	
}

pub mod dom_tree;
