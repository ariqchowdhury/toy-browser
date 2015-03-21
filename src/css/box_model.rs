pub struct Box {
	content: Rectangle,
	padding: Edges,
	border: Edges,
	margin: Edges,
}

pub struct Edges {
	top: f32,
	bottom: f32,
	right: f32,
	left: f32,
}

pub struct Rectangle {
	height: f32,
	width: f32,
}