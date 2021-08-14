use pyo3::prelude::*;
use pyo3::types::PyDict;

fn get_weight_matrix(py: &Python, locals: &PyDict) -> () {
    let code: &str = "np.array([[3, 2], [1, 4]])";
    let weights_matrix = py.eval(code, None,
                                 Some(&locals)).unwrap();
    locals.set_item("weights_matrix", weights_matrix);
}


fn invert_get_weight_matrix(py: &Python,
                            locals: &PyDict) -> () {
    let code: &str = "np.linalg.inv(weights_matrix)";
    let inverted_weights_matrix = py.eval(code, None,
                                  Some(&locals)).unwrap();
    locals.set_item("inverted_weights_matrix",
                    inverted_weights_matrix);
}


fn get_input_vector(py: &Python, locals: &PyDict,
                    first: i32, second: i32) -> () {
    let code: String = format!("np.array([[{}], [{}]])",
                               first, second);
    let input_vector = py.eval(&code.as_str(), None,
                               Some(&locals)).unwrap();
    locals.set_item("input_vector", input_vector);
}


fn get_times<'a>(py: &'a Python,
                 locals: &PyDict) -> &'a PyAny {
    let code: &str = "np.dot(weights_matrix, input_vector)";
    let times = py.eval(code, None, Some(&locals)).unwrap();
    return times
}

fn get_parameters<'a>(py: &'a Python,
                             locals: &PyDict) -> &'a PyAny {
    let code: &str = "
    np.dot(inverted_weights_matrix, input_vector)";
    let parameters = py.eval(code, None,
                             Some(&locals)).unwrap();
    return parameters
}


#[pyfunction]
pub fn calculate_times<'a>(result_dict: &'a PyDict,
    distance: i32, traffic_grade: i32) -> PyResult<&'a PyDict> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = PyDict::new(py);
    locals.set_item("np", py.import("numpy").unwrap());

    get_weight_matrix(&py, locals);
    get_input_vector(&py, locals, distance, traffic_grade);
    result_dict.set_item("times", get_times(&py, locals));
    return Ok(result_dict)
}


#[pyfunction]
pub fn calculate_parameters<'a>(result_dict: &'a PyDict,
    car_time: i32, truck_time: i32) -> PyResult<&'a PyDict> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = PyDict::new(py);
    locals.set_item("np", py.import("numpy").unwrap());

    get_weight_matrix(&py, locals);
    invert_get_weight_matrix(&py, locals);
    get_input_vector(&py, locals, car_time, truck_time);
    result_dict.set_item("parameters",
                         get_parameters(&py, locals));
    return Ok(result_dict)
}
