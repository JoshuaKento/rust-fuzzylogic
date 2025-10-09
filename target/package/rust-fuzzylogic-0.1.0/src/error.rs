//This File Defines The Basic Error Handling(Empty Input, Bad Arity,,, etc)
use std::error::Error;
use std::fmt;

//Basic Result-Type Definition For the functions in the library
pub type Result<T> = std::result::Result<T, FuzzyError>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
///Basic errors that can occur in the rust-fuzzylogic library
pub enum FuzzyError {
    BadArity,
    EmptyInput,
    TypeMismatch,
    OutOfBounds,
    NotFound { space: MissingSpace, key: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MissingSpace {
    Var,
    Input,
}

impl fmt::Display for FuzzyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FuzzyError::BadArity => {
                write!(f, "Bad arity")
            }
            FuzzyError::EmptyInput => {
                write!(f, "Empty input")
            }
            FuzzyError::TypeMismatch => {
                write!(f, "Invalid type input")
            }
            FuzzyError::OutOfBounds => {
                write!(f, "Out of bounds")
            }
            FuzzyError::NotFound { space, key } => {
                write!(
                    f,
                    "Keys not found. {key} cannot be found in {}",
                    match space {
                        MissingSpace::Input => "Inputs",
                        MissingSpace::Var => "Vars",
                    }
                )
            }
        }
    }
}

impl Error for FuzzyError {}

//Basic Unit Tests
#[cfg(test)]
mod tests {
    use crate::error::FuzzyError;
    #[test]
    fn print_error() {
        assert_eq!(FuzzyError::BadArity.to_string(), "Bad arity");
        assert_eq!(FuzzyError::EmptyInput.to_string(), "Empty input");
        assert_eq!(FuzzyError::TypeMismatch.to_string(), "Invalid type input");
        assert_eq!(FuzzyError::OutOfBounds.to_string(), "Out of bounds");
    }
}
