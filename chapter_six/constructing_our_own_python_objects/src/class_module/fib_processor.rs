use pyo3::prelude::{pyclass, pymethods};

use crate::fib_calcs::fib_number::fibonacci_number;
use crate::fib_calcs::fib_numbers::fibonacci_numbers;

#[pyclass]
pub struct FibProcessor {
    #[pyo3(get, set)]
    pub number: Vec<i32>,
    #[pyo3(get, set)]
    pub numbers: Vec<Vec<i32>>,
    #[pyo3(get)]
    pub number_results: Vec<u64>,
    #[pyo3(get)]
    pub numbers_results: Vec<Vec<u64>>,
}

#[pymethods]
impl FibProcessor {
    #[new]
    fn new(number: Vec<i32>, numbers: Vec<Vec<i32>>) -> Self {
        let input_numbers: Vec<Vec<i32>> = numbers.clone();
        let input_number: Vec<i32> = number.clone();

        let number_results: Vec<u64> = input_number.iter().map(|x| fibonacci_number(*x)).collect();

        let numbers_results: Vec<Vec<u64>> = Self::process_numbers(input_numbers);
        return FibProcessor {
            number,
            numbers,
            number_results,
            numbers_results,
        };
    }

    #[staticmethod]
    fn process_numbers(input_numbers: Vec<Vec<i32>>) -> Vec<Vec<u64>> {
        let mut buffer: Vec<Vec<u64>> = Vec::new();
        for i in input_numbers {
            buffer.push(fibonacci_numbers(i));
        }
        return buffer;
    }
}
