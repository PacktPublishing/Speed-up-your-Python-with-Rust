use super::traits;
use traits::{ClinicalSkills, AdvancedMedical, PatientRole};


pub fn admit_patient<Y: ClinicalSkills>(
    patient: &Box<dyn PatientRole>, _clinician: &Y) {
    println!("{} is being admitted", patient.get_name());
}


pub fn diagnose_patient<Y: AdvancedMedical>(
    patient: &Box<dyn PatientRole>, _clinician: &Y) {
    println!("{} is being diagnosed", patient.get_name());
}


pub fn prescribe_meds<Y: ClinicalSkills>(
    patient: &Box<dyn PatientRole>, clinician: &Y) {
    if clinician.can_prescribe() {
        println!("{} is being prescribed medication",
                 patient.get_name());
    } else {
        panic!("clinician cannot prescribe medication");
    }
}

pub fn administer_meds<Y: ClinicalSkills>(
    patient: &Box<dyn PatientRole>, _clinician: &Y) {
    println!("{} is having meds administered", patient.get_name());
}

pub fn discharge_patient<Y: ClinicalSkills>(
    patient: &Box<dyn PatientRole>, _clinician: &Y) {
    println!("{} is being discharged", patient.get_name());
}

