use std::io;
use std::io::prelude::*;


pub fn fibonacci_reccursive(n: i32) -> u64 {
	match n {
		1 | 2 => 1,
		_     => fibonacci_reccursive(n - 1) +
		         fibonacci_reccursive(n - 2)
	}
}


fn main() {
    let stdin = io::stdin();
    let stdout = std::io::stdout();
    let mut writer = stdout.lock();

    for line in stdin.lock().lines() {
        let input_int: i32 = line.unwrap().parse::<i32>()
                                          .unwrap();
        let fib_number = fibonacci_reccursive(input_int);
        writeln!(writer, "{}", fib_number);
    }
}
