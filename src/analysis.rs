use crate::data_model::OverdoseRecords;
use std::collections:;HashMap;

pub fn filter_stimulants(records: &[OverdoseRecord]) -> Vec<OverdoseRecord>{
    records
    .iter()
    .filter(|r| r.drug_type.contains("Stimulants"))
    .cloned()
    .collect()
}

pub fn rate_by_race_in_year(
    records: &[OverdoseRecord],
    year: u16,
) -> HashMap<String, f64> {
    let mut map = HashMap::new();
    for record in records.iter().filter(|r| r.year == year) { 
        map.insert(record.race_ethnicity.clone(), record.rate); 
    } 
    map 
}