use super::css::stylesheet;
use super::dom_tree;

/// A style node is used to create a parallel tree to the dom tree. Each
/// node contains a list of css declaration that would apply to the node 
pub struct StyleNode<'a> {
	element : &'a dom_tree::Element,
	pub declarations: Vec<stylesheet::Declaration>,
	pub children: Vec<StyleNode<'a>>,
}

impl<'a> StyleNode<'a> {

	pub fn new<'s>(node: &'s dom_tree::Element) -> StyleNode<'s> {
		let children = Vec::new();
		let decs = Vec::new();

		StyleNode {
			element: &node,
			declarations: decs,
			children: children,
		}
	}

	pub fn create_list_of_matching_declarations<'a>(&'a mut self, style: &'a stylesheet::StyleSheet) 
										-> Option<&Vec<stylesheet::Declaration>> {
		
		style.ruleset.rule_map.get(&stylesheet::Selector::SelectorType(self.element.e_type))
	}

}

