use super::css::stylesheet;
use super::dom_tree;

/// A style node is used to create a parallel tree to the dom tree. Each
/// node contains a list of css declaration that would apply to the node 
pub struct StyleNode<'a> {
	element : &'a dom_tree::Element,
	pub declarations: Option<&'a Vec<stylesheet::Declaration>>,
	pub children: Vec<StyleNode<'a>>,
}

impl<'a> StyleNode<'a> {

	pub fn new<'c>(node: &'c dom_tree::Element, style: &'c stylesheet::StyleSheet) -> StyleNode<'c> {
		let children = Vec::new();
		let decls = 
			style.ruleset.rule_map.get(&stylesheet::Selector::SelectorType(node.e_type));

		StyleNode {
			element: &node,
			declarations: decls,
			children: children,
		}
	}

	/// return a copy of the dom element contained by this style node
	pub fn get_element(& self) -> dom_tree::Element {
		(*self.element).clone()
	}
}

pub fn build_style_tree<'c>(root: &'c dom_tree::Element, 
							style: &'c stylesheet::StyleSheet) -> StyleNode<'c> {

	StyleNode {
		element: &root,
		declarations: style.
					  ruleset.
					  rule_map.
					  get(&stylesheet::Selector::SelectorType(root.e_type)),
		children: root.children.iter().map(|child| build_style_tree(child, style)).collect(),
	}

}

