use rust_project::{analysis, csv_reader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/Drug_overdose_death_rates__by_drug_type__sex__age__race__and_Hispanic_origin__United_States.csv";
    let all = csv_reader::read_records(path)?;

    {
        let mut labels: Vec<_> = all.iter()
            .map(|r| r.drug_type.clone())
            .collect();
        labels.sort();
        labels.dedup();
        eprintln!("Found {} unique drug_type labels:", labels.len());
        for lbl in &labels {
            eprintln!("  {}", lbl);
        }
    }