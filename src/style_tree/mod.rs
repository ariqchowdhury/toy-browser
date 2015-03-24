use super::css::stylesheet;
use super::css::stylesheet::{Property, BlockType};
use super::dom_tree;
use super::css::box_model;

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

	/// return a box_model::Box based on the stylesheet declarations of the
	/// StyleNode
	pub fn create_layout_box(& self) -> box_model::Box {
		let decls = self.declarations;
		let mut retval : box_model::Box;

		match decls {
			Some(d) => {
				let pt = stylesheet::box_value_from_declaration(d, Property::PaddingTop);
				let pb = stylesheet::box_value_from_declaration(d, Property::PaddingBottom);
				let pl = stylesheet::box_value_from_declaration(d, Property::PaddingLeft);
				let pr = stylesheet::box_value_from_declaration(d, Property::PaddingRight);

				let bt = stylesheet::box_value_from_declaration(d, Property::BorderTopHeight);
				let bb = stylesheet::box_value_from_declaration(d, Property::BorderBottomHeight);
				let bl = stylesheet::box_value_from_declaration(d, Property::BorderLeftHeight);
				let br = stylesheet::box_value_from_declaration(d, Property::BorderRightHeight);

				let mt = stylesheet::box_value_from_declaration(d, Property::MarginTop);
				let mb = stylesheet::box_value_from_declaration(d, Property::MarginBottom);
				let ml = stylesheet::box_value_from_declaration(d, Property::MarginLeft);
				let mr = stylesheet::box_value_from_declaration(d, Property::MarginRight);
				
				retval = box_model::Box::default();
				retval.padding.top = pt;
				retval.padding.bottom = pb;
				retval.padding.left = pl;
				retval.padding.right = pr;

				retval.border.top = bt;
				retval.border.bottom = bb;
				retval.border.left = bl;
				retval.border.right = br;

				retval.margin.top = mt;
				retval.margin.bottom = mb;
				retval.margin.left = ml;
				retval.margin.right = mr;
			}
			None => {retval = box_model::Box::default();}
		}

		retval
	}

	/// return a BlockType based on the stylesheet declaration of the
	/// StyleNode
	pub fn get_block_type(& self) -> BlockType {
		let decls = self.declarations;
		let mut retval : BlockType;

		match decls {
			Some(d) => {
				let display_val = d.iter().find(|x| x.property_name == Property::Display);

				match display_val {
					Some(d) => {
						match d.property_value {
							stylesheet::Value::Block(BlockType::Inline) => {retval = BlockType::Inline;}
							_ => {retval = BlockType::Block;}
						}
					}
					None => {retval = BlockType::Block;}
				}
			}
			None => {retval = BlockType::Block;}
		}

		retval
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

