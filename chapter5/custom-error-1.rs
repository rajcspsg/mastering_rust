
#[derive(Debug)]
enum Currency {
    USD,
    EUR,
}

#[derive(Debug)]
struct CurrencyError;

impl Currency {
    fn new(currency: &str) -> Result<Self, CurrencyError> {
        match currency {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            _ => Err(CurrencyError{})
        }
    }
}

#[derive(Debug)]
struct Money {
    currency: Currency,
    amount: u64
}

#[derive(Debug)]
struct MoneyError;

impl Money {
    fn new(currency: &str, amount: u64) -> Result<Self, MoneyError> {
        let currency = match Currency::new(currency) {
            Ok(c) => c,
            Err(_) => panic!("Unimplemented!!!")
        };
        Ok(Money{
            currency: currency,
            amount: amount
        })
    }
}
fn main() {
    let money_1 = Money::new("EUR", 12345);
    let money_2 = Money::new("FIM", 60000);

    println!("money_1 is {:?}", money_1);
    println!("money_2 is {:?}", money_2);
}