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

pub fn increase_since_2010(
    stimulant_records: &[OverdoseRecord],
) -> HashMap<String, f64> {
    let mut years: Vec<u16> = stimulant_records.iter().map(|r| r.year).collect();
    years.sort_unstable();
    years.dedup();
    let &latest_year = years.last().expect("No years in data");

    let base_map = rate_by_race_in_year(stimulant_records, 2010);
    let latest_map = rate_by_race_in_year(stimulant_records, latest_year);

    let mut diff = HashMap::new();
    for (race, &latest_rate) in &latest_map {
        if let Some(&base_rate) = base_map.get(race) {
            diff.insert(race.clone(), latest_rate - base_rate);
        }
    }
    diff
}
