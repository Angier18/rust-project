use crate::data_model::OverdoseRecord;
use csv::ReaderBuilder;
use std::error::Error;

pub fn read_records(path: &str) -> Result<Vec<OverdoseRecord>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let mut records = Vec::new();
    for result in reader.deserialize(){
        let record: OverdoseRecord = result?;
        records.push(record);}
    Ok(records)
}