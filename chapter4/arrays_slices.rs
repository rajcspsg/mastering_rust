fn main() {
    let mut integer_array_1 = [1,2,3];
    let integer_array_2:[u64; 3] = [2,3,4];
    let integer_array_3:[i32; 32] = [0;32];
    println!("{:?}", integer_array_2);
    println!("{:?}", integer_array_3);
    integer_array_1[1] = 255;

    println!("{:?}", integer_array_1);
}