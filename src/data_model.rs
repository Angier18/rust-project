//!In this mod I define each core part of my data type for my project, So I've called it OverdoseRecord and thats what adding what I need.
#[derive(Debug, Clone)]
pub struct OverdoseRecord {
    pub year: u16,
    pub drug_type: String,
    pub race_ethnicity: String,
    pub rate: f64,
}