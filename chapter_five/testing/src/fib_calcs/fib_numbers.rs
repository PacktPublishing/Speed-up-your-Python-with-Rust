use std::vec::Vec;
use pyo3::prelude::pyfunction;
use super::fib_number::fibonacci_number;


#[pyfunction]
pub fn fibonacci_numbers(numbers: Vec<i32>) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();

    for n in numbers.iter() {
        vec.push(fibonacci_number(*n));
    }
    return vec
}


#[cfg(test)]
mod fibonacci_numbers_tests {

	use super::fibonacci_numbers;

	#[test]
	fn test_run() {
		let outcome = fibonacci_numbers([1, 2, 3, 4].to_vec());
        assert_eq!(outcome, [1, 1, 2, 3]);
	}
}
