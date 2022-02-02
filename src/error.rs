#[derive(Debug, PartialEq)]
pub enum Error {
	UnexpectedToken,
}

impl Default for Error {
	fn default() -> Self {
		Error::UnexpectedToken
	}
}
