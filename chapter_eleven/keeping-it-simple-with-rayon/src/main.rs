extern crate rayon;

use rayon::prelude::*;


pub fn fibonacci_reccursive(n: i32) -> u64 {
	match n {
		1 | 2 => 1,
		_     => fibonacci_reccursive(n - 1) +
		         fibonacci_reccursive(n - 2)
	}
}


fn main() {
    // into_par_iter => 
    let numbers: Vec<u64> = vec![6, 7, 8, 9, 10].into_par_iter().map(
        |x| fibonacci_reccursive(x)
    ).collect();
    println!("{:?}", numbers);
    // In a lot of langues doing something like this is unthinkable but because of our borrowing rules in Rust we will not have data races
}
