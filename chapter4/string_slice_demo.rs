fn say_hello_slice(slice: &str) {
    println!("Hey {}", slice);
}

fn say_hello_string(string: &String) {
    println!("{:?}", string);
}

fn print_int(int_ref: &i32) {
    println!("{:?}", int_ref);
}
fn main() {
    let slice: &str = "you";
    let s: String = String::from("String");
    say_hello_slice(slice);
    say_hello_slice(&s);

    let number: i32 = 12345;
    print_int(&number);

    say_hello_string(&s);
}