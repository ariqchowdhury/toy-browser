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

pub struct Declaration {
	pub property_name : Property,
	pub property_value : Value,
}

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Property {
	FontSize,
	LineHeight,
	Color,
}

#[allow(dead_code)]
pub enum Value {
	Size(u32, Unit),
	Length(u32, Unit),
	ColorValue(Color),
	Placeholder,
}

#[allow(dead_code)]
pub enum Unit {
	Px,
}

#[allow(dead_code)]
pub struct Color {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
}

pub fn string_to_property(string :&str) -> Option<Property> {
	match string.trim() {
		"font-size" => Some(Property::FontSize),
		"line-height" => Some(Property::LineHeight),
		"color" => Some(Property::Color),
		_ => None,
	}
}

pub fn string_to_value(string :&str) -> Option<Value> {
	match string.trim() {
		_ => Some(Value::Placeholder),
	}
}

#[test]
fn test_string_to_property() {
	let prop = string_to_property(" font-size  ");
	assert!(prop.is_some());
	assert!(prop.unwrap() == Property::FontSize);

	let invalid_prop = string_to_property(" sdf");
	assert!(invalid_prop.is_none());
}

#[test]
fn test_string_to_value() {
	let val = string_to_value("sdlfj");
	assert!(val.is_some());
}