use text_parser;
use std::collections::HashMap;

use dom_tree;

/// A stylesheet contains the rules to apply to the DOM
pub struct StyleSheet {
	pub ruleset : Rule,
}

/// A Rule is a dict key'd by a selector, and storing a 
/// list of declarations
pub struct Rule {
	pub rule_map: HashMap<Selector, Vec<Declaration>>,
}

impl Rule {
	pub fn new() -> Rule {
		let rule_map = HashMap::new();

		Rule {
			rule_map: rule_map,
		}
	}
}

/// A Selector is an element to which style rules apply
/// www.w3.org/TR/CSS2/selector.html
/// Only TypeSelectors are currently supported, and are
/// implemented by matching element types from dom_tree 
#[derive(Debug, Copy, PartialEq, Hash, Eq)]
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
#[derive(Debug, PartialEq, Copy)]
pub enum Property {
	FontSize,
	LineHeight,
	Color,
	MarginTop,
	MarginBottom,
	MarginLeft,
	MarginRight,
	PaddingTop,
	PaddingBottom,
	PaddingLeft,
	PaddingRight,
	BorderTopHeight,
	BorderBottomHeight,
	BorderLeftHeight,
	BorderRightHeight,
}

/// Supported CSS values to apply to Properties
#[allow(dead_code)]
#[derive(PartialEq, Copy)]
pub enum Value {
	Size(u32, Unit),
	ColorValue(Color),
	Placeholder,
}

/// Supported units of measurement for CSS
#[derive(PartialEq, Copy)]
pub enum Unit {
	Px,
	Em,
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
		"margin-top" => Some(Property::MarginTop),
		"margin-bottom" => Some(Property::MarginBottom),
		"margin-left" => Some(Property::MarginLeft),
		"margin-right" => Some(Property::MarginRight),
		"padding-top" => Some(Property::PaddingTop),
		"padding-bottom" => Some(Property::PaddingBottom),
		"padding-left" => Some(Property::PaddingLeft),
		"padding-right" => Some(Property::PaddingRight),
		"border-top-height" => Some(Property::BorderTopHeight),
		"border-bottom-height" => Some(Property::BorderBottomHeight),
		"border-left-height" => Some(Property::BorderLeftHeight),
		"border-right-height" => Some(Property::BorderRightHeight),
		_ => None,
	}
}

/// Take a string and match to the Value type. Return None
/// if no match
pub fn string_to_value(string :&str) -> Option<Value> {
	let input = string.trim().to_string();
	let mut parse = text_parser::TextParser::new(input);

	match parse.peek_char() {
		None => Some(Value::Placeholder),
		Some(c) if c.is_whitespace() => Some(Value::Placeholder),
		Some(c) if match c {'0'...'9' => true, _ => false,} => {

			let num = parse.consume_while(|c| match c {
				'0'...'9' => true,
				_ => false,
			});

			let cur_char = parse.peek_char();
			let next_char = parse.peek_next_char();

			let are_units = || cur_char.is_some() && 
								next_char.is_some();

			if are_units() {
				let is_pixel = || {cur_char.unwrap() == 'p' && 
								   next_char.unwrap() == 'x'};
				let is_em = || {cur_char.unwrap() == 'e' && 
								next_char.unwrap() == 'm'};

				let unit = if is_pixel() {
					Unit::Px
				} else if is_em() {
					Unit::Em
				} else {
					Unit::Px
				};

				match num.parse::<u32>().ok() {
					Some(n) => Some(Value::Size(n, unit)),
					None => Some(Value::Placeholder)
				}
				
			} else {
				Some(Value::Placeholder)
			}

		}
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