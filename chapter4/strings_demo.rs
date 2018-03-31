fn main() {
    
    let mut empty_string = String::new();
    let mut empty_string_with_capacity = String::with_capacity(50);
    let mut string_from_byte_string: String = String::from_utf8(vec![82, 83, 84, 85]).expect("Creating string from byte string failed");

    println!("length of the string is {}", empty_string.len());
    println!("length of the string with capacity {}", empty_string_with_capacity.len());
    println!("length of the string from byte string is {}", string_from_byte_string.len());

    println!("empty string says {:?}", empty_string);
    println!("empty_string_with_capacity says {}", empty_string_with_capacity);
    println!("string_from_byte_string says {}", string_from_byte_string);

    empty_string.push('1');
    println!("empty string says {:?}", empty_string);
    println!("length of the string is {}", empty_string.len());

    empty_string.push_str("2345");
    println!("empty string says {:?}", empty_string);
    println!("length of the string is {}", empty_string.len());

    empty_string_with_capacity.push('1');
    println!("empty_string_with_capacity says {}", empty_string_with_capacity);
    println!("length of the string from byte string is {}", empty_string_with_capacity.len());
    empty_string_with_capacity.push_str("2345");
    println!("length of the string from byte string is {}", empty_string_with_capacity.len());
    println!("empty_string_with_capacity says {}", empty_string_with_capacity);
}