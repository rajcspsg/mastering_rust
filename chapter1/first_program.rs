fn main() {
    let target_inferrred = "inferred world";
    let target: &'static str = "non inferred world";

    println!("Hi there, {}", target_inferrred);
    println!("Hi there {}", target);
}
