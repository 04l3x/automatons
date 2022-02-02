use crate::automaton::Automaton;
use crate::error::Error;
use crate::token::{Token, TokenKind};

///Camila recognize numbers
pub struct Camila;
pub enum CamilaState {
	Initial,
	CollectingIntegers,
	CollectingDecimals,
}

impl Automaton for Camila {
	type State = CamilaState;

	fn run(input: &'static str, initial_state: Self::State) -> Result<Token, Error> {
		let mut current_state = initial_state;
		let mut value = String::new();

		for i in input.chars() {
			match i {
				'0' => match current_state {
					CamilaState::Initial => {}
					_ => value.push(i),
				},

				'1'..='9' => match current_state {
					CamilaState::Initial => {
						current_state = CamilaState::CollectingIntegers;
						value.push(i);
					}
					_ => value.push(i),
				},

				'.' => match current_state {
					CamilaState::Initial => {
						current_state = CamilaState::CollectingDecimals;
						value.push('0');
						value.push(i);
					}
					CamilaState::CollectingIntegers => {
						current_state = CamilaState::CollectingDecimals;
						value.push(i);
					}
					CamilaState::CollectingDecimals => return Err(Error::default()),
				},
				_ => return Err(Error::default()),
			}
		}

		match current_state {
			CamilaState::Initial => Err(Error::default()),
			_ => Ok(Token::new(value, TokenKind::Number)),
		}
	}
}


#[cfg(test)]
mod camila_tests {
	use super::{Automaton, Error, Camila, CamilaState, Token, TokenKind};

	#[test]
	fn integers() {
		let inputs = ["12345", "0250", "000200", "1960"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Camila::run(i, CamilaState::Initial));
		}

		let expected = [
			Ok(Token::new(String::from("12345"), TokenKind::Number)),
			Ok(Token::new(String::from("250"), TokenKind::Number)),
			Ok(Token::new(String::from("200"), TokenKind::Number)),
			Ok(Token::new(String::from("1960"), TokenKind::Number)),
		];

		assert_eq!(outputs, expected);
	}


	#[test]
	fn decimals() {
		let inputs = ["12.345", "02.50", "000.200", ".1960"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Camila::run(i, CamilaState::Initial));
		}

		let expected = [
			Ok(Token::new(String::from("12.345"), TokenKind::Number)),
			Ok(Token::new(String::from("2.50"), TokenKind::Number)),
			Ok(Token::new(String::from("0.200"), TokenKind::Number)),
			Ok(Token::new(String::from("0.1960"), TokenKind::Number)),
		];

		assert_eq!(outputs, expected);
	}


	#[test]
	fn decimals_and_integers() {
		let inputs = ["12.345", "000032445", "02.50", "9800011", "000.200", "00100020", ".1960"];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(Camila::run(i, CamilaState::Initial));
		}

		let expected = [
			Ok(Token::new(String::from("12.345"), TokenKind::Number)),
			Ok(Token::new(String::from("32445"), TokenKind::Number)),
			Ok(Token::new(String::from("2.50"), TokenKind::Number)),
			Ok(Token::new(String::from("9800011"), TokenKind::Number)),
			Ok(Token::new(String::from("0.200"), TokenKind::Number)),
			Ok(Token::new(String::from("100020"), TokenKind::Number)),
			Ok(Token::new(String::from("0.1960"), TokenKind::Number)),
		];

		assert_eq!(outputs, expected);
	}
}
