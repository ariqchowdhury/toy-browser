pub struct Box {
	pub content: Rectangle,
	pub padding: Edges,
	pub border: Edges,
	pub margin: Edges,
}

impl Box {
	pub fn default() -> Box {
		Box {
			content: Rectangle {x: 0, y: 0, height: 0, width: 0},
			padding: Edges {top: 0, bottom: 0, right: 0, left: 0},
			border: Edges {top: 0, bottom: 0, right: 0, left: 0},
			margin: Edges {top: 0, bottom: 0, right: 0, left: 0},
		}
	}

}

pub struct Edges {
	pub top: u32,
	pub bottom: u32,
	pub right: u32,
	pub left: u32,
}

pub struct Rectangle {
	/// x position of the rectangle, relative to a (0,0) origin at
	/// the top left of the parent container
	pub x: u32,
	/// y position of the rectangle, relative to a (0,0) origin at
	/// the top left of the parent container
	pub y: u32,

	pub height: u32,
	pub width: u32,
}