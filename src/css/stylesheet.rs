use super::dom_tree;

struct StyleSheet {
	ruleset : Vec<Rule>,
}

struct Rule {
	selector : Vec<Selector>,
	declaration : Vec<Declaration>,
}

struct Selector {
	type_selector : ElementType,
}

struct Declaration {
	property_name : Property,
	property_value : Value,
}

enum Property {
	FontSize,
	LineHeight,
	Color,
}

enum Value {
	Size(Unit),
	Length(Unit),
	ColorValue(Color),
}

enum Unit {
	Px,
}

struct Color {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
}