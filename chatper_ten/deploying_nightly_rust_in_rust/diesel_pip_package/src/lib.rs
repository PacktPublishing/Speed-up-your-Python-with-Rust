#[macro_use] extern crate diesel;
extern crate dotenv;
use diesel::prelude::*;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;

mod database;
mod schema;
mod models;

use database::establish_connection;
use models::FibEntry;
use schema::fib_entries;


#[pyfunction]
fn get_fib_entries(url: String, py: Python) -> Vec<&PyDict> {

   let connection = establish_connection(url);

   let fibs = fib_entries::table
       .order(fib_entries::columns::input_number.asc())
       .load::<FibEntry>(&connection)
       .unwrap();

   let mut buffer = Vec::new();

   for i in fibs {
       let placeholder = PyDict::new(py);
       placeholder.set_item("input number",
                            i.input_number.unwrap());
       placeholder.set_item("fib number",
                            i.calculated_number.unwrap());
       buffer.push(placeholder);
   }
   return buffer
}


#[pymodule]
fn rust_db_cloning(_py: Python, m: &PyModule)
                                   -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_fib_enteries));
    Ok(())
}
