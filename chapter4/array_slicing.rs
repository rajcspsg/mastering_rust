        use std::fmt::Debug;

        fn print_slice<T: Debug>(slice: &[T]) {
            println!("{:?}", slice);
        }

        fn main() {
            println!("{:?}", "array_slicing");
            let array:[u8; 5] = [1,2,3,4,5];
            println!("{:?}", "whole array just burrowed");
            print_slice(&array);

            println!("Whole array sliced");
            print_slice(&array[..]);

            println!("Slicing without the last index");
            print_slice(&array[1..]);
            
            println!("Slicing with both first and last index");
            print_slice(&array[2..4]);

            println!("Slicing without the last index");
            print_slice(&array[..3]);

            println!("oops, thats too far");
            print_slice(&array[..9]);
        }