use crate::error::Error;
use crate::token::Token;

pub trait Automaton {
	type State;

	fn run(input: &'static str, initial_state: Self::State) -> Result<Token, Error>;
}
