use super::css::stylesheet;
use super::dom_tree;

/// A style node is used to create a parallel tree to the dom tree. Each
/// node contains a list of css declaration that would apply to the node 
// TODO(ARIQ): Might need to have the actual node (ptr to node) in this 
// struct in order to render it...
pub struct StyleNode {
	pub declarations: Vec<stylesheet::Declaration>,
	pub children: Vec<StyleNode>,
}

impl StyleNode {

	pub fn new() -> StyleNode {
		let children = Vec::new();
		let decs = Vec::new();

		StyleNode {
			declarations: decs,
			children: children,
		}
	}
}

fn does_node_match_rule(rule: stylesheet::Rule, node: dom_tree::Element) -> bool {
	false
}

fn create_list_of_matching_declarations(rules: Vec<stylesheet::Rule>) -> Vec<stylesheet::Declaration> {
	let mut retval = Vec::new();
	let dec = rules[0].declaration;
	retval.push(dec);

	retval
}