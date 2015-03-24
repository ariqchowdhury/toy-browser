#[allow(dead_code)]
pub struct Box {
	pub content: Rectangle,
	pub padding: Edges,
	pub border: Edges,
	pub margin: Edges,
}

impl Box {
	pub fn default() -> Box {
		Box {
			content: Rectangle {height: 0, width: 0},
			padding: Edges {top: 0, bottom: 0, right: 0, left: 0},
			border: Edges {top: 0, bottom: 0, right: 0, left: 0},
			margin: Edges {top: 0, bottom: 0, right: 0, left: 0},
		}
	}

}

#[allow(dead_code)]
pub struct Edges {
	pub top: u32,
	pub bottom: u32,
	pub right: u32,
	pub left: u32,
}

#[allow(dead_code)]
pub struct Rectangle {
	height: u32,
	width: u32,
}