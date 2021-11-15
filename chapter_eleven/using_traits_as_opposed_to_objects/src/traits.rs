

pub trait Speak {
    fn introduce(&self) -> ();
}


pub trait ClinicalSkills {
    fn can_prescribe(&self) -> bool {
        return false
    }
    fn can_diagnose(&self) -> bool {
        return false
    }
    fn can_administer_medication(&self) -> bool {
        return true
    }
}


pub trait AdvancedMedical {}

pub trait PatientRole {
    fn get_name(&self) -> String;
}

