use crate::data_model::OverdoseRecord;
use csv::{ReaderBuilder, StringRecord};
use std::error::Error;

pub fn read_records(path: &str) -> Result<Vec<OverdoseRecord>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let headers = reader.headers()?.clone();
    let panel_i = headers.iter().position(|h| h == "PANEL").unwrap();
    let label_i = headers.iter().position(|h| h == "STUB_LABEL").unwrap();
    let year_i  = headers.iter().position(|h| h == "YEAR").unwrap();
    let rate_i  = headers.iter().position(|h| h == "ESTIMATE").unwrap();

    let mut records = Vec::new();
    for result in reader.records() {
        let row: StringRecord = result?;

        let year_str  = row.get(year_i).unwrap().trim();
        let raw_drug = row.get(panel_i).unwrap().trim();
        let drug_str = if raw_drug == "All drug overdose deaths" {
            "All drug overdose deaths: unnamed drug"
        } else {
            raw_drug
        };        let race_str  = row.get(label_i).unwrap().trim();
        let rate_str  = row.get(rate_i).unwrap().trim();

        if year_str.is_empty()
            || drug_str.is_empty()
            || race_str.is_empty()
            || rate_str.is_empty()
        {
            continue;
        }
        let rec = OverdoseRecord {
            year:           year_str.parse()?,
            drug_type:      drug_str.to_string(),
            race_ethnicity: race_str.to_string(),
            rate:           rate_str.parse()?,
        };
        records.push(rec);
    }

    Ok(records)
}
