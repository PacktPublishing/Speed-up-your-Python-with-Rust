use std::env;
use std::vec::Vec;


pub fn fibonacci_number(n: i32) -> u64 {
	if n < 0 {
		panic!("{} is negative!", n);
	}
	match n {
		0     => panic!("zero is not a right argument to
		                fibonacci_number!"),
		1 | 2 => 1,
		_     => fibonacci_number(n - 1) +
		         fibonacci_number(n - 2)
	}
}


pub fn fibonacci_numbers(numbers: Vec<i32>) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();

    for n in numbers.iter() {
        vec.push(fibonacci_number(*n));
    }
    return vec
}



fn main() {
    let mut inputs: Vec<i32> = Vec::new();

    let args: Vec<String> = env::args().collect();

    for i in args {
        match i.parse::<i32>() {
            Ok(result) => inputs.push(result),
            Err(_) => (),
        }
    }

    let results = fibonacci_numbers(inputs);

    for i in results {
        println!("{}", i);
    }
}
