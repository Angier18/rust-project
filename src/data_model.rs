use serde::Deserialize;

#[derive(Debug,Deserialize, Clone)]

pub struct OverdoseRecord {
    pub YEAR: u16,
    pub drug_type: String,
    pub STUB_LABEL: String,
    pub rate: f64,
}