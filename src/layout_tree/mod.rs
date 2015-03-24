use super::style_tree;
use super::css::box_model;
use super::css::stylesheet;

pub struct LayoutNode<'a> {
	layout_box : box_model::Box,
	block_type : stylesheet::BlockType,
	content : &'a style_tree::StyleNode<'a>,
	pub children : Vec<LayoutNode<'a>>,
}

pub fn build_layout_tree<'b>(style: &'b style_tree::StyleNode<'b>) -> LayoutNode<'b> {
	
	LayoutNode {
		layout_box: style.create_layout_box(),
		block_type: style.get_block_type(),
		content: &style,
		children: style.children.iter().map(|child| { build_layout_tree(child)}).collect(),
	}
}

impl<'a> LayoutNode<'a> {

	/// Layout a layout tree by calculating widths down the layout tree
	/// and calculating heights on the way back up. Widths are dependant on 
	/// the parent width while heights depend on a node's children.
	/// Width dependancy says width cannot be larger than parent width.
	/// Height dependancy says a parent's height is the sum of all children's 
	/// height plus its own. 
	/// The parent_dimension for the root of a layout tree is the viewport
	/// of the browser window.
	pub fn layout(&mut self, parent_dimension: box_model::Rectangle) {
		match self.block_type {
			stylesheet::BlockType::Block => self.layout_block(parent_dimension),
			stylesheet::BlockType::Inline => {}
			_ => {}
		}
	}

	/// Calculate self width based off layout_box widths. Ensure width does not
	/// exceed parent width. Pass self's content down. 
	fn layout_block(&mut self, parent_dimension: box_model::Rectangle) {
		self.calculate_width(parent_dimension);
	}

	/// Calculate width of a box, constraining for parent width. Set self
	/// box_model content width. 
	/// Constraing algorithm is loosely based off www.w3.org/TR/CSS2/visudet.html#blockwidth
	/// We will just calculate if our width is greater than parent; if so, margin-right is set
	/// to 0 and we will reduce the width to be equal to parent width
	fn calculate_width(&mut self, parent_dimension: box_model::Rectangle) {

		let mut initial_width = self.layout_box.padding.right + self.layout_box.padding.left +
				 	      		self.layout_box.border.right + self.layout_box.border.left +
					      		self.layout_box.margin.right + self.layout_box.margin.left;

		if initial_width > parent_dimension.width {
			self.layout_box.margin.right = 0;
			initial_width = parent_dimension.width;
		}

		self.layout_box.content.width = initial_width;
	}

	/// Calculate the position of this content. It will be based off the parent
	/// position, and then modified by self's top and left properties
	fn calculate_position(&mut self, parent_dimension: box_model::Rectangle) {
		self.layout_box.content.x = parent_dimension.x +
									self.layout_box.padding.left +
									self.layout_box.margin.left +
									self.layout_box.border.left;

		self.layout_box.content.y = parent_dimension.y +
									self.layout_box.padding.top +
									self.layout_box.margin.top +
									self.layout_box.border.top;
	}
}
