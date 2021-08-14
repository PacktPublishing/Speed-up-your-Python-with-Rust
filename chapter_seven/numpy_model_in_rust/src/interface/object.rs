use pyo3::prelude::{pyfunction, PyResult, Python};
use pyo3::types::{PyAny, PyDict};
use pyo3::exceptions::PyLookupError;

use super::config::run_config;


fn extract_data<'a>(input_object: &'a PyAny, attribute: &'a str, config_dict: &'a PyDict) -> &'a PyDict {
    match input_object.getattr(attribute) {
        Ok(data) => {
            config_dict.set_item(attribute, data).unwrap();
        },
        Err(_) => Err(PyLookupError::new_err(
            "attribute number is missing")).unwrap()
    }
    return config_dict
}



#[pyfunction]
pub fn object_interface<'a>(input_object: &'a PyAny) -> PyResult<&'a PyAny> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let mut config_dict: &PyDict = PyDict::new(py);

    config_dict = extract_data(input_object, "number", config_dict);
    config_dict = extract_data(input_object, "numbers", config_dict);
    let output_dict: &PyDict = run_config(config_dict).unwrap();

    input_object.setattr("number_results", output_dict.get_item("NUMBER RESULT").unwrap()).unwrap();
    input_object.setattr("numbers_results", output_dict.get_item("NUMBERS RESULT").unwrap()).unwrap();

    return Ok(input_object)

}

