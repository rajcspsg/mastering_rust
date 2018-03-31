use std::convert::Into;

#[derive(Debug)]
struct Money<T> {
    amount: T,
    currency: String
}

#[derive(Debug)]
struct CurrencylessMoney<T> {
    amount: T
}
impl<T> Into<CurrencylessMoney<T>> for Money<T> {
    fn into(self) -> CurrencylessMoney<T> {
        CurrencylessMoney{amount: self.amount}
    }
}
fn main() {
    let money = Money{amount: 32, currency: "USD".to_string()};
    let currencylessMoney: CurrencylessMoney<u32> = money.into();
    println!("currencylessMoney {:?} for money  is", currencylessMoney);
}