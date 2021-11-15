use super::traits;
use traits::PatientRole;


pub struct PatientList {
    pub patients: Vec<Box<dyn PatientRole>>
}
