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
