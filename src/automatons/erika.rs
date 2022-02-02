use crate::automaton::Automaton;
use crate::error::Error;
use crate::token::{Token, TokenKind};

///Erika recognize strings
pub struct Erika;

pub enum ErikaState {
	Initial,
	Collecting,
	Finished,
}

impl Automaton for Erika {
	type State = ErikaState;

	fn run(input: &'static str, initial_state: Self::State) -> Result<Token, Error> {
		let mut current_state = initial_state;
		let mut token_value: String = String::new();
		for i in input.chars() {
			match i {
				'"' => match current_state {
					ErikaState::Initial => {
						current_state = ErikaState::Collecting;
					}
					ErikaState::Collecting => {
						current_state = ErikaState::Finished;
					}
					ErikaState::Finished => {
						return Err(Error::UnexpectedToken);
					}
				},
				_ => match current_state {
					ErikaState::Initial => {
						return Err(Error::UnexpectedToken);
					}
					ErikaState::Collecting => {
						token_value.push(i);
					}
					ErikaState::Finished => {
						return Err(Error::UnexpectedToken);
					}
				},
			}
		}

		match current_state {
			ErikaState::Finished => Ok(Token::new(token_value, TokenKind::Text)),
			_ => Err(Error::UnexpectedToken),
		}
	}
}

#[cfg(test)]
mod erika_tests {
	use super::{Automaton, Erika, ErikaState, Error, Token, TokenKind};

	#[test]
	fn empty_string() {
		let output = Erika::run("\"\"", ErikaState::Initial);

		let expected = Ok(Token::new(String::from(""), TokenKind::Text));

		assert_eq!(output, expected);
	}

	#[test]
	fn some_words() {
		let inputs = ["\"fuck\"", "\"weed\"", "\"and\"", "\"beer\""];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Erika::run(i, ErikaState::Initial));
		}

		let expected = [
			Ok(Token::new(String::from("fuck"), TokenKind::Text)),
			Ok(Token::new(String::from("weed"), TokenKind::Text)),
			Ok(Token::new(String::from("and"), TokenKind::Text)),
			Ok(Token::new(String::from("beer"), TokenKind::Text)),
		];

		assert_eq!(outputs, expected);
	}

	#[test]
	fn without_quotation_marks() {
		let inputs = ["fuck", "weed", "and", "beer"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Erika::run(i, ErikaState::Initial));
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
	fn unclosed_collect() {
		let inputs = ["\"fuck", "\"weed", "\"and", "\"beer"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Erika::run(i, ErikaState::Initial));
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
	fn unstart_string() {
		let inputs = ["fuck\"", "weed\"", "and\"", "beer\""];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Erika::run(i, ErikaState::Initial));
		}

		let expected = [
			Err(Error::default()),
			Err(Error::default()),
			Err(Error::default()),
			Err(Error::default()),
		];

		assert_eq!(outputs, expected);
	}
}
