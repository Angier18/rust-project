//!we use the functions from the other .rs, does clustering for the avg, prints the charts and stats, loads the overdose data, and filters to different groups 
use rust_project::{analysis, csv_reader};
use rust_project::data_model::OverdoseRecord;

fn print_cluster(title: &str, bucket: &[(String, f64)]) {
    println!("  {} ({}):", title, bucket.len());
    if bucket.is_empty() {
        println!("    (nothing)");
    } else {
        for (race, delta) in bucket {
            println!("    - {:<25} {:+.2}", race, delta);
        }
    }
}//this prints the cluster the above and below avg of their race to change in rate pairs by ofrmating each line with signs and two decimals

fn run_analysis_and_plot(// here it runs the change in rate for the analysis and prints a sorted table and its statistics.
    label: &str,
    subset: &[OverdoseRecord],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n==== Analysis for: {} ====", label);
    if subset.is_empty() {
        println!("  No records.");
        return Ok(());
    }

    let diffs = analysis::increase_since_2010(subset);
    if diffs.is_empty() {
        println!("  Not enough years.");
        return Ok(());//this part of the code does the math to make a mao of race to change in rate
    }

    let mut sorted: Vec<(&String, &f64)> = diffs.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap()); //we will sort by highest change in rate to lowest

    println!("  Race/Ethnicity                 Change in rate since 2010");
    println!("  ----------------------------------------------");
    for (race, delta) in &sorted {
        println!("  {:<30} {:+.2}", race, *delta); //so we will print the header for the chart and print eeach race and delta
    }

    if let Some(&(race, delta)) = sorted.first() {
        println!("\n  Highest change in rate: {} at +{:.2}", race, delta); //making sure that the highest will have a seperate line for it so its easier to read and know the highest.
    }

    let total: f64 = diffs.values().sum();
    let avg = total / (diffs.len() as f64);
    println!("\n  Average change in rate: {:.2}", avg); // here we do the math to find the avg change in rate

    let (high, low): (Vec<_>, Vec<_>) =
        diffs.clone().into_iter().partition(|(_, v)| *v >= avg);
    print_cluster("Above avg", &high);
    print_cluster("Below avg", &low);

    Ok(()) //so we will split the avg into above and below
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/Drug_overdose_death_rates__by_drug_type__sex__age__race__and_Hispanic_origin__United_States.csv";
    let all = csv_reader::read_records(path)?;

    let demo_only: Vec<OverdoseRecord> = all //here only some iterators chain for filtering so we keep only "male" and "female"
        .into_iter()
        .filter(|r| {
            let l = r.race_ethnicity.to_lowercase();
            (l.starts_with("male:") || l.starts_with("female:"))
                && !l.chars().any(|c| c.is_ascii_digit())
        })
        .collect();

    let mut drugs: Vec<_> = demo_only.iter().map(|r| r.drug_type.clone()).collect();
    drugs.sort();
    drugs.dedup();
    for drug in &drugs {
        let subset: Vec<_> = demo_only
            .iter()
            .filter(|r| &r.drug_type == drug)
            .cloned()
            .collect();
        run_analysis_and_plot(drug, &subset)?;
    }

    run_analysis_and_plot("All Drugs Combined", &demo_only)?;
    Ok(())
}// here we will load the records from the csv thats located in data and filter to make and femake deographics, using emumerate each drug type and run analysis in turn to finallyty combining that whole column and doing a chart for all the drugs.