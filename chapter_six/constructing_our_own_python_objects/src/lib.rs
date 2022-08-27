use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod class_module;
mod fib_calcs;
mod interface;

use class_module::fib_processor::FibProcessor;
use fib_calcs::fib_number::fibonacci_number;
use fib_calcs::fib_numbers::fibonacci_numbers;
use interface::config::run_config;

#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust!");
}

#[pymodule]
fn flitton_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello))?;
    m.add_wrapped(wrap_pyfunction!(fibonacci_number))?;
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers))?;
    m.add_wrapped(wrap_pyfunction!(run_config))?;
    m.add_class::<FibProcessor>()?;
    Ok(())
}
