use dom_tree;

/// A stylesheet contains the rules to apply to the DOM
#[allow(dead_code)]
pub struct StyleSheet {
	pub ruleset : Vec<Rule>,
}

impl StyleSheet {
	pub fn new() -> StyleSheet {
		let ruleset = Vec::new();

		StyleSheet {
			ruleset: ruleset,
		}
	}
}

/// A Rule is a tuple of a selector and its matching
/// declaration 
#[allow(dead_code)]
pub struct Rule {
	pub selector: Selector,
	pub declaration: Declaration,
}

/// A Selector is an element to which style rules apply
/// www.w3.org/TR/CSS2/selector.html
/// Only TypeSelectors are currently supported, and are
/// implemented by matching element types from dom_tree 
#[derive(Debug, Copy)]
#[derive(PartialEq)]
pub enum Selector {
	SelectorType(dom_tree::ElementType),
}

/// A declaration is the CSS property and value to 
/// apply to a selector. 
/// www.w3.org/TR/CSS2/syndata.html#declaration
#[derive(Copy)]
pub struct Declaration {
	pub property_name : Property,
	pub property_value : Value,
}

/// Supported CSS properties
#[allow(dead_code)]
#[derive(Debug, PartialEq, Copy)]
pub enum Property {
	FontSize,
	LineHeight,
	Color,
}

/// Supported CSS values to apply to Properties
#[allow(dead_code)]
#[derive(PartialEq, Copy)]
pub enum Value {
	Size(u32, Unit),
	Length(u32, Unit),
	ColorValue(Color),
	Placeholder,
}

/// Supported units of measurement for CSS
#[derive(PartialEq, Copy)]
#[allow(dead_code)]
pub enum Unit {
	Px,
}

#[derive(PartialEq, Copy)]
#[allow(dead_code)]
pub struct Color {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
}

/// Take a string and match to the Property type. Return None
/// if no match
pub fn string_to_property(string :&str) -> Option<Property> {
	match string.trim() {
		"font-size" => Some(Property::FontSize),
		"line-height" => Some(Property::LineHeight),
		"color" => Some(Property::Color),
		_ => None,
	}
}

/// Take a string and match to the Value type. Return None
/// if no match
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