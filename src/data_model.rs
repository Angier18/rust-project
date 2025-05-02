//!In this mod I define each core part of my data type for my project, So I've called it OverdoseRecord and thats what adding what I need.
#[derive(Debug, Clone)]
pub struct OverdoseRecord {
    pub year: u16,
    pub drug_type: String,
    pub race_ethnicity: String,
    pub rate: f64,
}// for this struc as stated represents a single row of the overdose stats and it has the year, drug, race, and rate

#[allow(dead_code)]
pub enum AnalysisMode {
    Absolute,
    Relative,
}

//!here we all this dead code to be left alone so we declare a pubic enumeration thats called AnalysisMode that has two veriants absolute and relative, so if later I want to pass a analysismode value inot functions. SO it helps answer how have overdose rates chnaged. Abs: latest rate - baseline rate and Relative: latest rate - baseline rate / baseline*100%. its like an extendstion to our analysis code and helps with measuring how the rates have evolved 