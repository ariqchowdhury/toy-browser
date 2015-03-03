use super::parser;

pub struct HtmlParser {
	pub parse: parser::Parser,	
}

impl HtmlParser {
	pub fn new(parser: parser::Parser) -> HtmlParser {
		HtmlParser {
			parse: parser,
		}
	}


}