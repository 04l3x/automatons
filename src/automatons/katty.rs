use crate::automaton::Automaton;
use crate::error::Error;
use crate::token::{Token, TokenKind};

///Katty recognize identifiers and keywords
pub struct Katty;
pub enum KattyState {
	Initial,
	Collecting,
	StopCollecting,
}

impl Automaton for Katty {
	type State = KattyState;

	fn run(input: &'static str, initial_state: Self::State) -> Result<Token, Error> {
		let mut current_state = initial_state;
		let mut value = String::new();

		for i in input.chars() {
			match i {
				'_' | 'A'..='Z' | 'a'..='z' => match current_state {
					KattyState::Initial => {
						current_state = KattyState::Collecting;
						value.push(i);
					}
					KattyState::Collecting => {
						value.push(i);
					}
					KattyState::StopCollecting => return Err(Error::default()),
				},
				'0'..='9' => match current_state {
					KattyState::Initial => return Err(Error::default()),
					KattyState::Collecting => {
						value.push(i);
					}
					KattyState::StopCollecting => return Err(Error::default()),
				},
				' ' | '\t' | '\n' | '\0' => match current_state {
					KattyState::Collecting => {
						current_state = KattyState::StopCollecting;
					}
					_ => {}
				},
				_ => return Err(Error::default()),
			}
		}

		match current_state {
			KattyState::Collecting | KattyState::StopCollecting => match Keyword::get(&value) {
				Some(_) => Ok(Token::new(value, TokenKind::Keyword)),
				None => Ok(Token::new(value, TokenKind::Identifier)),
			},
			_ => Err(Error::default()),
		}
	}
}

enum Keyword {
	Yellow,
	Wall,
	Eyes,
}

impl Keyword {
	fn get<'k>(key: &'k str) -> Option<Keyword> {
		match key {
			"yellow" => Some(Keyword::Yellow),
			"wall" => Some(Keyword::Wall),
			"eyes" => Some(Keyword::Eyes),
			_ => None,
		}
	}
}

#[cfg(test)]
mod katty_tests {
	use super::{Automaton, Error, Katty, KattyState, Token, TokenKind};

	#[test]
	fn identifiers() {
		let inputs = ["run", "write", "drink", "Number"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Katty::run(i, KattyState::Initial));
		}

		let expected = [
			Ok(Token::new(String::from("run"), TokenKind::Identifier)),
			Ok(Token::new(String::from("write"), TokenKind::Identifier)),
			Ok(Token::new(String::from("drink"), TokenKind::Identifier)),
			Ok(Token::new(String::from("Number"), TokenKind::Identifier)),
		];

		assert_eq!(outputs, expected);
	}

	#[test]
	fn identifiers_with_spaces() {
		let inputs = ["  run ", "  write    ", "drink    ", "    Number"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Katty::run(i, KattyState::Initial));
		}

		let expected = [
			Ok(Token::new(String::from("run"), TokenKind::Identifier)),
			Ok(Token::new(String::from("write"), TokenKind::Identifier)),
			Ok(Token::new(String::from("drink"), TokenKind::Identifier)),
			Ok(Token::new(String::from("Number"), TokenKind::Identifier)),
		];

		assert_eq!(outputs, expected);
	}

	#[test]
	fn bad_identifiers() {
		let inputs = ["ru  n ", "  wr ite    ", "drin k", "    N umber"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Katty::run(i, KattyState::Initial));
		}

		let expected = [
			Err(Error::default()),
			Err(Error::default()),
			Err(Error::default()),
			Err(Error::default()),
		];

		assert_eq!(outputs, expected);
	}

	#[test]
	fn keywords() {
		let inputs = ["yellow", "wall", "eyes"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Katty::run(i, KattyState::Initial));
		}

		let expected = [
			Ok(Token::new(String::from("yellow"), TokenKind::Keyword)),
			Ok(Token::new(String::from("wall"), TokenKind::Keyword)),
			Ok(Token::new(String::from("eyes"), TokenKind::Keyword)),
		];

		assert_eq!(outputs, expected);
	}

	#[test]
	fn keywords_with_spaces() {
		let inputs = ["  yellow  ", "wall  ", "   eyes"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Katty::run(i, KattyState::Initial));
		}

		let expected = [
			Ok(Token::new(String::from("yellow"), TokenKind::Keyword)),
			Ok(Token::new(String::from("wall"), TokenKind::Keyword)),
			Ok(Token::new(String::from("eyes"), TokenKind::Keyword)),
		];

		assert_eq!(outputs, expected);
	}

	#[test]
	fn bad_keywords() {
		let inputs = ["yel low", " wa ll ", " eye s"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Katty::run(i, KattyState::Initial));
		}

		let expected = [
			Err(Error::default()),
			Err(Error::default()),
			Err(Error::default()),
		];

		assert_eq!(outputs, expected);
	}

	#[test]
	fn identifiers_and_keywords() {
		let inputs = ["yellow", "boot", "wall", "weed"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Katty::run(i, KattyState::Initial));
		}

		let expected = [
			Ok(Token::new(String::from("yellow"), TokenKind::Keyword)),
			Ok(Token::new(String::from("boot"), TokenKind::Identifier)),
			Ok(Token::new(String::from("wall"), TokenKind::Keyword)),
			Ok(Token::new(String::from("weed"), TokenKind::Identifier)),
		];

		assert_eq!(outputs, expected);
	}
}
