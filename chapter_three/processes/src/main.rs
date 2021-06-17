use std::time;
use rayon::prelude::*;


pub fn fibonacci_reccursive(n: i32) -> u64 {
	if n < 0 {
		panic!("{} is negative!", n);
	}
	match n {
		0     => panic!("zero is not a right argument to fibonacci_reccursive()!"),
		1 | 2 => 1,
        3     => 2,
		_     => fibonacci_reccursive(n - 1) + fibonacci_reccursive(n - 2)
	}
}


fn main() {
    let now = time::Instant::now();
    fibonacci_reccursive(8);
    fibonacci_reccursive(12);
    fibonacci_reccursive(12);
    fibonacci_reccursive(20);
    fibonacci_reccursive(20);
    fibonacci_reccursive(20);
    fibonacci_reccursive(20);
    fibonacci_reccursive(28);
    fibonacci_reccursive(28);
    fibonacci_reccursive(28);
    fibonacci_reccursive(28);
    fibonacci_reccursive(36);
    fibonacci_reccursive(46);
    fibonacci_reccursive(46);
    fibonacci_reccursive(46);
    println!("time elapsed {:?}", now.elapsed());
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();

    let now = time::Instant::now();
    let numbers: Vec<i32> = vec![8, 12, 12, 20, 20, 20, 20, 28, 28, 28, 28, 36, 46, 46, 46];
    let outcomes: Vec<u64> = numbers.into_par_iter().map(|n| fibonacci_reccursive(n)).collect();
    println!("{:?}", outcomes);
    println!("time elapsed {:?}", now.elapsed());
}
