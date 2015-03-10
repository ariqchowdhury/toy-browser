use dom_tree;

#[allow(dead_code)]
struct StyleSheet {
	ruleset : Vec<Rule>,
}

#[allow(dead_code)]
struct Rule {
	selector : Vec<Selector>,
	declaration : Vec<Declaration>,
}

#[derive(Debug, Copy)]
pub enum Selector {
	SelectorType(dom_tree::ElementType),
}

#[allow(dead_code)]
struct Declaration {
	property_name : Property,
	property_value : Value,
}

#[allow(dead_code)]
enum Property {
	FontSize,
	LineHeight,
	Color,
}

#[allow(dead_code)]
enum Value {
	Size(Unit),
	Length(Unit),
	ColorValue(Color),
}

#[allow(dead_code)]
enum Unit {
	Px,
}

#[allow(dead_code)]
struct Color {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
}