use crate::automaton::Automaton;
use crate::error::Error;
use crate::token::Token;

///For unreconigzed words, always returns error
pub struct TheGirlOfTheWhiteDog;
pub enum TheGirlOfTheWhiteDogState {
	Initial,
	Error,
}

impl Automaton for TheGirlOfTheWhiteDog {
	type State = TheGirlOfTheWhiteDogState;

	fn run(input: &'static str, initial_state: Self::State) -> Result<Token, Error> {
		let mut current_state = initial_state;
		for _ in input.chars() {
			match current_state {
				TheGirlOfTheWhiteDogState::Initial => {
					current_state = TheGirlOfTheWhiteDogState::Error;
				}
				_ => {}
			}
		}

		Err(Error::default())
	}
}

#[cfg(test)]
mod the_girl_of_the_white_dog_tests {
	use super::{Automaton, Error, TheGirlOfTheWhiteDog, TheGirlOfTheWhiteDogState, Token};

	#[test]
	fn always_fails() {
		let inputs = ["?hello", "12345", "+-**/", "yellow", "\"blue moon\""];
		let mut outputs: Vec<Result<Token, Error>> = vec![];

		for i in inputs {
			outputs.push(TheGirlOfTheWhiteDog::run(
				i,
				TheGirlOfTheWhiteDogState::Initial,
			));
		}

		let expected = [
			Err(Error::default()),
			Err(Error::default()),
			Err(Error::default()),
			Err(Error::default()),
			Err(Error::default()),
		];

		assert_eq!(outputs, expected);
	}
}
