use text_parser;
use dom_tree;
use super::stylesheet;

#[allow(dead_code)]
struct CssParser {
	pub parse: text_parser::TextParser,
}

#[allow(dead_code)]
impl CssParser {
	pub fn new(parser: text_parser::TextParser) -> CssParser {
		CssParser {
			parse: parser,
		}
	}

	fn parse_selector(&mut self) -> Option<stylesheet::Selector> {
		let selector = self.parse.consume_while(|c| !c.is_whitespace() && c != '{');

		match selector.as_slice() {
			"title" => Some(stylesheet::Selector::SelectorType(dom_tree::ElementType::Title)),
			"body" => Some(stylesheet::Selector::SelectorType(dom_tree::ElementType::Body)),
			"h1" | 
			"h2" | 
			"h3" | 
			"h4" => Some(stylesheet::Selector::SelectorType(dom_tree::ElementType::Head)),
			_ => None,
		}
	}
}