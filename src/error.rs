use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ListError {}

impl fmt::Display for ListError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "An error occured, probably out of bounds")
	}
}

impl Error for ListError {}
