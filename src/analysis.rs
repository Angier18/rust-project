//! I wrote functions for the overdoserecords, first filter_by_keyword finds all the records where drug type contains a keyword, then we have the function increase since 2010 which does the math for chaing in ratae from 2010 to 2018 which is the lastest year, then giving us a map from race/ethnicity to change in rate.
use crate::data_model::OverdoseRecord;
use std::collections::HashMap; // put the utilities needed

pub fn filter_by_keyword(
    records: &[OverdoseRecord],
    keyword: &str,
) -> Vec<OverdoseRecord> {
    let kw = keyword.to_lowercase();
    records.iter()
        .filter(|r| r.drug_type.to_lowercase().contains(&kw))
        .cloned() //we will filter using iterator chains
        .collect() // here i would only return the drug type with keywprds that is also case insensitive so that I can seperate the drug types for the chart
}

pub fn rate_by_race_in_year(
    records: &[OverdoseRecord],
    year: u16,
) -> HashMap<String, f64> {
    let mut map = HashMap::new();
    for rec in records.iter().filter(|r| r.year == year) {//now we group by race/ethnicity
        map.insert(rec.race_ethnicity.clone(), rec.rate);
    }
    map
} //here it returns a Hashmap mapping each demographic group to its overdose rate in that year so it inputs the sluce of my datatset and year and returns the keys in strings for that create vy returning a completed map

pub fn increase_since_2010(
    records: &[OverdoseRecord],
) -> HashMap<String, f64> {
    let mut years: Vec<u16> = records.iter().map(|r| r.year).collect();
    years.sort_unstable();
    years.dedup();
    let latest = *years.last().expect("Nothing in data");

    let base   = rate_by_race_in_year(records, 2010);
    let latest = rate_by_race_in_year(records, latest);

    let mut diff = HashMap::new();
    for (race, &lrate) in &latest {
        if let Some(&brate) = base.get(race) {
            diff.insert(race.clone(), lrate - brate);
        }
    }
    diff
}} //here for each race/ethnicity we will compute the rate in the latest year (2018) - rate in 2010 which includesboth the 2010 baseline first we will group by recorsing all by race/ethnicity then funding the rate in 2010 if aby and finding the record with the max year > 2010 finally recording the computed differemce.
