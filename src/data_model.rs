use serde::Deserialize;

#[derive(Debug,Deserialize, Clone)]
pub struct OverdoseRecord {
    #[serde(rename = "YEAR")]
    pub year: u16,

    #[serde(rename = "STUB_LABEL")]
    pub drug_type: String,
    
    #[serde(rename = "STUB_NAME")]
    pub race_ethnicity: String,

    #[serde(rename = "ESTIMATE")]
    pub rate: f64,
}