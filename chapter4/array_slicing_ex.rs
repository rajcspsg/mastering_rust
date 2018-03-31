fn main() {
    let array:[u32; 6] = [1,2,3,4,5,6];
    let mut sum = 0;
    for x in array.iter() {
        sum += x;
    }
    println!("sum is {}", sum);

    let array_ref: &[u32] = &array;
    sum = 0;
    for x in array_ref {
        sum += x;
    }
    println!("sum is {}", sum);

    sum = 0;
    for x in &array {
        sum += x;
    }
    println!("sum is {}", sum);
}