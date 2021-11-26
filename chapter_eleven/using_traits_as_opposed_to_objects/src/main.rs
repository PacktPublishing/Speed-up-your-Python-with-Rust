mod traits;
mod objects;
mod people;
mod actions;

use people::{Patient, Nurse, Doctor};
use objects::PatientList;
use actions::{admit_patient, diagnose_patient, prescribe_meds,
              administer_meds, discharge_patient};


fn main() {
    let doctor = Doctor{name: String::from("Torath")};
    let doctor_two = Doctor{name: String::from("Sergio")};
    let nurse = Nurse{name: String::from("Maxwell")};
    let nurse_two = Nurse{name: String::from("Nathan")};

    let patient_list = PatientList {
        patients: vec![
            Box::new(Patient{name: String::from("pestilence")}),
            Box::new(Patient{name: String::from("war")}),
            Box::new(Patient{name: String::from("famine")}),
            Box::new(Patient{name: String::from("death")})
        ]
    }
    for i in patient_list.patients {
        admit_patient(&i, &nurse);
        diagnose_patient(&i, &doctor);
        prescribe_meds(&i, &doctor_two);
        administer_meds(&i, &nurse_two);
        discharge_patient(&i, &nurse);
    }
}
