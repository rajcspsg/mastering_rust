fn select_first<T>(first:T, _:T) -> T {
    first
}

fn main() {
    let first = select_first(1,2);
    println!("first {}", first);   
    println!("{:?}", select_first("meow", "grrr...")); 
}