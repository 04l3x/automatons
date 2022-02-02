#[derive(Debug, PartialEq)]
pub struct Token {
	value: String,
	kind: TokenKind,
}

impl Token {
	pub fn new(value: String, kind: TokenKind) -> Token {
		Token { value, kind }
	}
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
	Text,
	Identifier,
	Keyword,
	Number,
}
