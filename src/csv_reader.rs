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
    for result in reader.deserialize(){
        let record: OverdoseRecord = result?;
        records.push(record);}
    Ok(records)
}