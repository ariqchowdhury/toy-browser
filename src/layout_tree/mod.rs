use super::style_tree;
use super::css::box_model;
use super::css::stylesheet;

pub struct LayoutNode<'a> {
	layout_box : box_model::Box,
	block : stylesheet::BlockType,
	content : &'a style_tree::StyleNode<'a>,
	pub children : Vec<LayoutNode<'a>>,
}

pub fn build_layout_tree<'b>(style: &'b style_tree::StyleNode<'b> ) -> LayoutNode<'b> {
	
	LayoutNode {
		layout_box: style.create_layout_box(),
		block: style.get_block_type(),
		content: &style,
		children: style.children.iter().map(|child| { build_layout_tree(child)}).collect(),
	}
}
