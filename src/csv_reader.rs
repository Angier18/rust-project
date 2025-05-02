//!Here it reads the CSV file, looks at the header, skips blank rows - becasue it has no data, normalizes "all drug overdose" label, makes each row inoto an OverdoseRecord.
use crate::data_model::OverdoseRecord;
use csv::{ReaderBuilder, StringRecord};
use std::error::Error; //I put the utilities needed for this mod

pub fn read_records(path: &str) -> Result<Vec<OverdoseRecord>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()//I will open the CSV reader with header 
        .has_headers(true)
        .from_path(path)?;

    let headers = reader.headers()?.clone(); //here we get each header row that we can see the columns indiced by name
    let panel_i = headers.iter().position(|h| h == "PANEL").unwrap();
    let label_i = headers.iter().position(|h| h == "STUB_LABEL").unwrap();
    let year_i  = headers.iter().position(|h| h == "YEAR").unwrap();
    let rate_i  = headers.iter().position(|h| h == "ESTIMATE").unwrap();

    let mut records = Vec::new();
    for result in reader.records() {// parsing loop and using data transformation
        let row: StringRecord = result?;// here we parse one csv row and then for each field we will compute the index

        let year_str = row.get(year_i).unwrap().trim();
        let raw_drug = row.get(panel_i).unwrap().trim();
        let race_str = row.get(label_i).unwrap().trim();
        let rate_str = row.get(rate_i).unwrap().trim();

        if year_str.is_empty() || raw_drug.is_empty() || race_str.is_empty() || rate_str.is_empty() {
            continue;// we will skip the rows with missing information becasue its useless for our studies
        }

        let drug_str = if raw_drug == "All drug overdose deaths" {
            "All drug overdose deaths: unnamed drug"
        } else {
            raw_drug
        }; //we normalize the "all drug overdose deaths" becasue it will give us a picture of the unnamed drugs and shows us a more descriptive label for ir

        let rec = OverdoseRecord {
            year:           year_str.parse()?,
            drug_type:      drug_str.to_string(),
            race_ethnicity: race_str.to_string(),
            rate:           rate_str.parse()?,
        };
        records.push(rec); //here we parse into a struct
    }

    Ok(records)
}