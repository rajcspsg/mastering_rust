use std::fmt::Result;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;

#[derive(Debug)]
struct MoneyError;

impl Display for MoneyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MoneyError due to {}", self.cause)
    }
}

impl Error for MoneyError {
    fn description( &self) -> &str {
        "MoneyError"
    }
    fn cause(&self) -> Option<&Error> {
        Some(&self.cause)
    }
}
