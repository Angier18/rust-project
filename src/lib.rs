//!here it reads the raw CSV data into overdoserecord stucts, has the test, helps main.rs, does the math for per-demograhic rate, and finally checks records.
pub mod data_model;
pub mod csv_reader;
pub mod analysis;
//this crate will make the .rs modules that are needed. for example using data_model it will cintian the most important type used for the whole project. Csv reader will add the csv parsing logic, and analysis will provide the calculations and data transformations.

#[cfg(test)]
mod tests { //this code will bring testing targets inoto scope
    use super::analysis::{filter_by_keyword, increase_since_2010};
    use super::data_model::OverdoseRecord;

    #[test] //its checks theat filter by keyword correctly picks only the records that drug type has the given substring
    fn test_filter_by_keyword() {
        let records = vec![
            OverdoseRecord {
                year: 2010,
                drug_type: "Stimulants Test".into(),
                race_ethnicity: "GroupA".into(),
                rate: 1.0, // so we will arrange into two records that the drug type contians the giving substring, then only one contains stimulant
            },
            OverdoseRecord {
                year: 2010,
                drug_type: "OtherDrug".into(),
                race_ethnicity: "GroupB".into(),
                rate: 2.0,
            },
        ];
        let filtered = filter_by_keyword(&records, "stimulant");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].race_ethnicity, "GroupA"); //acts as a filter for the keyword stimulant, then assert exactly one record remianis which is gruopA
    }

    #[test]
    fn test_increase_since_2010() {
        let records = vec![
            OverdoseRecord {
                year: 2010,
                drug_type: "X".into(),
                race_ethnicity: "A".into(),
                rate: 1.0,
            },
            OverdoseRecord {
                year: 2020, // latest
                drug_type: "X".into(),
                race_ethnicity: "A".into(),
                rate: 3.0,
            },
            OverdoseRecord {
                year: 2020,
                drug_type: "X".into(),
                race_ethnicity: "B".into(),
                rate: 2.0,
            },
        ];
        let diffs = increase_since_2010(&records);
        assert_eq!(diffs.get("A"), Some(&2.0));
        assert!(diffs.get("B").is_none());
    }
} //here it verifies that the increase snce 2010 function does the math the rate change form 2010 to 2018 and leaves any demographic without a 2010 baseline. we kind of do the same thing which set year to 2020 which is latest for A and then we know that B has no baseline, finallt it computes the differences amd a's change in rate is 2 and b is not includeed because 2010 has no record.