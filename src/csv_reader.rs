use crate::data_model::OverdoseRecord;
use csv::ReaderBuilder;
use std::error::Error;

pub fn read_records(path: &str) -> Result<Vec<OverdoseRecord>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let headers = reader.headers()?.clone();
    let year_i  = headers.iter().position(|h| h == "YEAR").unwrap();
    let label_i = headers.iter().position(|h| h == "STUB_LABEL").unwrap();
    let name_i  = headers.iter().position(|h| h == "STUB_NAME").unwrap();
    let rate_i  = headers.iter().position(|h| h == "ESTIMATE").unwrap();

    let mut records = Vec::new();
    for result in reader.records(){
        let row: StringRecord = result?;
        let rate_str = row.get(rate_i).unwrap().trim();
        if rate_str.is_empty() {
            continue;  // skip blank rates
        }
        let rec = OverdoseRecord {
            year:           row.get(year_i).unwrap().parse()?,
            drug_type:      row.get(label_i).unwrap().to_string(),
            race_ethnicity: row.get(name_i).unwrap().to_string(),
            rate:           rate_str.parse()?,
        };
        records.push(rec);
    }
    Ok(records)
}