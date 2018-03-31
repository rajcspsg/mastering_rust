use std::ops::Add;

#[derive(Debug)]
struct Money<T> {
    amount: T,
    currency: String
}

impl<T: Add<T, Output=T>> Add for Money<T> {
    type Output = Money<T>;
    fn add(self, rhs: Money<T>) -> Self::Output {
        assert!(self.currency == rhs.currency);
        Money{amount: self.amount + rhs.amount, currency: self.currency}
    }
}

fn main() {
    let whole_eros: Money<u8> = Money{amount: 42, currency: "EU".to_string()};
    let floating_euros:Money<f32> = Money{amount: 12.5, currency: "EU".to_string()};
    println!("whole euros {:?}", whole_eros);
    println!("floating euros {:?}", floating_euros);

    let whole_eros_1: Money<u8> = Money{amount: 42, currency: "EU".to_string()};
    let whole_eros_2: Money<u8> = Money{amount: 42, currency: "EU".to_string()};
    let summed_euros = whole_eros_1 + whole_eros_2;
    println!("summed_euros {:?}", summed_euros);
}