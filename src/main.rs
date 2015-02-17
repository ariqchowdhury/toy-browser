extern crate browser;

use browser::dom_tree;

fn main() {
	let document = dom_tree::Document::new(dom_tree::Doctype::Html);
	//let root_element = dom_tree::Element::new_root(dom_tree::ClassE); // dont need, make doc does it
}