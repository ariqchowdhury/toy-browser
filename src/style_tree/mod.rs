use super::css::stylesheet;

/// A style node is used to create a parallel tree to the dom tree. Each
/// node contains a list of css declaration that would apply to the node 
pub struct StyleNode {
	pub declarations: Vec<stylesheet::Declaration>,
	pub children: Vec<StyleNode>,
}