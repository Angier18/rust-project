
pub mod data_model;
pub mod csv_reader;
pub mod analysis;

#[cfg(test)]
mod tests {
    use super::analysis::{filter_by_keyword, increase_since_2010};
    use super::data_model::OverdoseRecord;

    #[test]
    fn test_filter_by_keyword() {
        let records = vec![
            OverdoseRecord {
                year: 2010,
                drug_type: "Stimulants Test".into(),
                race_ethnicity: "GroupA".into(),
                rate: 1.0,
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
        assert_eq!(filtered[0].race_ethnicity, "GroupA");
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
}
