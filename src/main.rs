use rust_project::{analysis, csv_reader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/Drug_overdose_death_rates__by_drug_type__sex__age__race__and_Hispanic_origin__United_States.csv";
    let all = csv_reader::read_records(path)?;

    let stim = analysis::filter_by_keyword(&all, "stimulant");if stim.is_empty() {
    eprintln!("❗️ No records matched “stimulant”. \
               Make sure your keyword matches one of:\n\
               {}", 
               all.iter()
                  .map(|r| r.drug_type.clone())
                  .collect::<std::collections::HashSet<_>>()
                  .into_iter()
                  .fold(String::new(), |acc, lbl| acc + "  • " + &lbl + "\n")
    );
    std::process::exit(1);
    }
    
    let diff_map = analysis::increase_since_2010(&stim);

    let mut sorted: Vec<_> = diff_map.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    println!("Race/Ethnicity                     Change in rate since 2010");
    println!("-----------------------------------------------");
    for (race, delta) in &sorted {
        println!("{:<30} {:+.2}", race, delta);
    }
    if let Some((top, val)) = sorted.first() {
    println!("\n Largest increase: {} at +{:.2}", top, *val);
    }

    let total: f64 = diff_map.values().sum();
    let avg = total / (diff_map.len() as f64);
    let (high, low): (Vec<_>, Vec<_>) = diff_map
    .into_iter()
    .partition(|(_, v)| *v >= avg);

    println!("\nAverage increase across groups: +{:.2}", avg);
    println!("Above-average cluster:");
    for (r, _) in &high {
        println!("  {}", r);
    }
    println!("Below-average cluster:");
    for (r, _) in &low {
        println!("  {}", r);
    }

    Ok(())
}