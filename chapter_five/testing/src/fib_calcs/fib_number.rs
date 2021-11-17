use pyo3::prelude::pyfunction;


#[pyfunction]
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


#[cfg(test)]
mod fibonacci_number_tests {
	use super::fibonacci_number;

	#[test]
	fn test_one() {
		assert_eq!(fibonacci_number(1), 1);
	}
	#[test]
	fn test_two() {
		assert_eq!(fibonacci_number(2), 1);
	}
	#[test]
	fn test_three() {
		assert_eq!(fibonacci_number(3), 2);
	}
	#[test]
	fn test_twenty() {
		assert_eq!(fibonacci_number(20), 6765);
	}
	#[test]
	#[should_panic]
	fn test_0() {
		fibonacci_number(0);
	}
	#[test]
     #[should_panic]
     fn test_negative() {
         fibonacci_number(-20);
     }
}

