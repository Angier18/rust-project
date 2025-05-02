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
    let stim = analysis::filter_stimulants(&all);

    let increases = analysis::increase_since_2010(&stim);
    let mut inc_vec: Vec<_> = increases.into_iter().collect();
    inc_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("Race/Ethnicity                     Δ Rate since 2010");
    println!("-----------------------------------------------");
    for (race, delta) in inc_vec {
        println!("{:<30} {:+.2}", race, delta);
    }

    Ok(())
}
