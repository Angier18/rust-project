use crate::data_model::OverdoseRecord;
use std::collections::HashMap;

pub fn filter_by_keyword(
    records: &[OverdoseRecord],
    keyword: &str,
) -> Vec<OverdoseRecord> {
    let kw = keyword.to_lowercase();
    records.iter()
        .filter(|r| r.drug_type.to_lowercase().contains(&kw))
        .cloned()
        .collect()
}

pub fn rate_by_race_in_year(
    records: &[OverdoseRecord],
    year: u16,
) -> HashMap<String, f64> {
    let mut map = HashMap::new();
    for rec in records.iter().filter(|r| r.year == year) {
        map.insert(rec.race_ethnicity.clone(), rec.rate);
    }
    map
}

pub fn increase_since_2010(
    records: &[OverdoseRecord],
) -> HashMap<String, f64> {
    let mut years: Vec<u16> = records.iter().map(|r| r.year).collect();
    years.sort_unstable();
    years.dedup();
    let latest = *years.last().expect("No years in data");

    let base   = rate_by_race_in_year(records, 2010);
    let latest = rate_by_race_in_year(records, latest);

    // subtract
    let mut diff = HashMap::new();
    for (race, &lrate) in &latest {
        if let Some(&brate) = base.get(race) {
            diff.insert(race.clone(), lrate - brate);
        }
    }
    diff
}