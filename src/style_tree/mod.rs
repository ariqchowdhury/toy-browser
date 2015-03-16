use super::css::stylesheet;
use super::dom_tree;

/// A style node is used to create a parallel tree to the dom tree. Each
/// node contains a list of css declaration that would apply to the node 
pub struct StyleNode<'a, 'b> {
	element : &'a dom_tree::Element,
	pub declarations: Option<&'b Vec<stylesheet::Declaration>>,
	pub children: Vec<StyleNode<'a, 'b>>,
}

impl<'a, 'b> StyleNode<'a, 'b> {

	pub fn new<'c, 'd>(node: &'c dom_tree::Element, style: &'d stylesheet::StyleSheet) -> StyleNode<'c, 'd> {
		let children = Vec::new();
		let decls = 
			style.ruleset.rule_map.get(&stylesheet::Selector::SelectorType(node.e_type));

		StyleNode {
			element: &node,
			declarations: decls,
			children: children,
		}
	}
}

