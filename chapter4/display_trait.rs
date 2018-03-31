use std::fmt::{Formatter, Display, Result};

#[derive(Debug)]
struct Money<T> {
    amount: T,
    currency: String
}

impl<T: Display> Display for Money<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt , "Money {} {}", self.amount, self.currency)
    }
}

fn main() {
    let money = Money{amount: 42, currency: "EUR".to_string()};
    println!("Displaying Money \n {}", money);
}