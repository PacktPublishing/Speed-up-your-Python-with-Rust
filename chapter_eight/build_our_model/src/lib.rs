use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;

mod footprint;
mod vulnerabilities;

use footprint::merge_event_ids_with_footprint;
use vulnerabilities::merge_vulnerabilities_with_footprint;
use vulnerabilities::structs::VulnerabilityFootPrint;


#[pyfunction]
fn get_model(event_ids: Vec<i32>, mut base_path: String, py: Python) -> Vec<&PyDict> {
    let footprints = merge_event_ids_with_footprint(event_ids, base_path.clone());
    let model = merge_vulnerabilities_with_footprint(footprints, base_path);

   let mut buffer = Vec::new();

   for i in model {
       let placeholder = PyDict::new(py);
       placeholder.set_item("vulnerability_id", i.vulnerability_id);
       placeholder.set_item("intensity_bin_id", i.intensity_bin_id);
       placeholder.set_item("damage_bin_id", i.damage_bin_id);
       placeholder.set_item("damage_probability", i.damage_probability);
       placeholder.set_item("event_id", i.event_id);
       placeholder.set_item("areaperil_id", i.areaperil_id);
       placeholder.set_item("footprint_probability", i.footprint_probability);
       placeholder.set_item("total_probability", i.total_probability);
       buffer.push(placeholder);
   }
   return buffer
}

#[pymodule]
fn flitton_oasis_risk_modelling(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_model));
    Ok(())
}
