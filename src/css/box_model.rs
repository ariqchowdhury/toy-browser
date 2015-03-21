#[allow(dead_code)]
pub struct Box {
	content: Rectangle,
	padding: Edges,
	border: Edges,
	margin: Edges,
}

#[allow(dead_code)]
pub struct Edges {
	top: u32,
	bottom: u32,
	right: u32,
	left: u32,
}

#[allow(dead_code)]
pub struct Rectangle {
	height: u32,
	width: u32,
}