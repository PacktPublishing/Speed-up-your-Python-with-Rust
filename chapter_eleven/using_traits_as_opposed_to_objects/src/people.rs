use super::traits;
use traits::{Speak, ClinicalSkills, AdvancedMedical, PatientRole};


pub struct Patient {
    pub name: String
}

pub struct Nurse {
    pub name: String
}

pub struct NursePractitioner {
    pub name: String
}

pub struct AdvancedNursePractitioner {
    pub name: String
}

pub struct Doctor {
    pub name: String
}


impl Speak for Patient {
    fn introduce(&self) {
        println!("hello I'm a Patient and my name is {}",
                 self.name);
    }
}

impl Speak for Nurse {
    fn introduce(&self) {
        println!("hello I'm a Nurse and my name is {}",
                 self.name);
    }
}

impl Speak for NursePractitioner {
    fn introduce(&self) {
        println!("hello I'm a Practitioner and my name is {}",
                  self.name);
    }
}

impl PatientRole for Patient {
    fn get_name(&self) -> String {
        return self.name.clone()
    }
}
impl ClinicalSkills for Nurse {}

impl ClinicalSkills for NursePractitioner {
    fn can_prescribe(&self) -> bool {
        return true
    }
}


impl AdvancedMedical for AdvancedNursePractitioner {}
impl AdvancedMedical for Doctor {}

impl<T> ClinicalSkills for T where T: AdvancedMedical {
    fn can_prescribe(&self) -> bool {
        return true
    }
    fn can_diagnose(&self) -> bool {
        return true
    }
}
