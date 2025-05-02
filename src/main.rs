use rust_project::{analysis, csv_reader};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/Drug_overdose_death_rates__by_drug_type__sex__age__race__and_Hispanic_origin__United_States.csv";
    let all = csv_reader::read_records(path)?;

    // gather all unique drug types
    let mut drug_types: Vec<_> = all.iter()
        .map(|r| r.drug_type.clone())
        .collect();
    drug_types.sort();
    drug_types.dedup();
    
    for drug in drug_types {
    println!("\n==== Analysis for: {} ====", drug);

    // filter to exactly this drug
    let subset: Vec<_> = all.iter()
        .filter(|r| r.drug_type == drug)
        .cloned()
        .collect();

    if subset.is_empty() {
        println!("  No records for this category.\n");
        continue;
    }

    // compute Δ since 2010
    let diffs = analysis::increase_since_2010(&subset);
    if diffs.is_empty() {
        println!("  Not enough years of data to compute a Δ.\n");
        continue;
    }

    // build & sort the list of (race, delta)
    let mut sorted: Vec<_> = diffs.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    // **FULL TABLE PRINT** (re-added!)
    println!("  Race/Ethnicity                     Δ Rate since 2010");
    println!("  -----------------------------------------------");
    for (race, delta) in &sorted {
        println!("  {:<30} {:+.2}", race, *delta);
    }

    // print the single top‐riser
    if let Some((race, &Δ)) = sorted.first() {
        println!("\n  🏆 Largest increase: {} at +{:.2}", race, Δ);
    }

    // compute average & clusters
    let total: f64 = diffs.values().sum();
    let avg = total / (diffs.len() as f64);
    println!("  Average Δ for this drug: +{:.2}", avg);

    let (high, low): (Vec<_>, Vec<_>) =
        diffs.into_iter().partition(|(_, &v)| v >= avg);

    // pretty-print clusters
    print_cluster("Above average", &high);
    print_cluster("Below average", &low);
}


// helper to pretty-print a cluster
fn print_cluster(title: &str, bucket: &[(String, f64)]) {
    if bucket.is_empty() {
        println!("   {}: (none)", title);
    } else {
        println!("   {}:", title);
        for (race, Δ) in bucket {
            println!("     • {:<20} +{:.2}", race, Δ);
        }
    }
}
