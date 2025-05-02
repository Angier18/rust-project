use rust_project::{analysis, csv_reader};
use plotters::prelude::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/Drug_overdose_death_rates__by_drug_type__sex__age__race__and_Hispanic_origin__United_States.csv";
    let all = csv_reader::read_records(path)?;
    let mut overall_best = ("<none>".to_string(), std::f64::MIN);
    let mut drug_types: Vec<_> = all.iter().map(|r| r.drug_type.clone()).collect();
    drug_types.sort();
    drug_types.dedup();

    for drug in drug_types {
        println!("\n==== Analysis for: {} ====", drug);
        let subset: Vec<_> = all.iter()
            .filter(|r| r.drug_type == drug)
            .cloned()
            .collect();
        if subset.is_empty() {
            println!("  No records for this category.");
            continue;
        }

        // Compute Δ since 2010
        let diffs = analysis::increase_since_2010(&subset);
        if diffs.is_empty() {
            println!("  Not enough years of data to compute a Δ.");
            continue;
        }

        // Build & sort a Vec of (&race, &delta)
        let mut sorted: Vec<_> = diffs.iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        // 1) Print full table
        println!("  Race/Ethnicity                     Δ Rate since 2010");
        println!("  -----------------------------------------------");
        for (race, delta) in &sorted {
            println!("  {:<30} {:+.2}", race, *delta);
        }

        // 2) Top-riser for this drug
        if let Some((race, &Δ)) = sorted.first() {
            println!("\n  Largest increase: {} at +{:.2}", race, Δ);

            // Update overall if this Δ is the best so far
            if Δ > overall_best.1 {
                overall_best = (race.clone(), Δ);
            }
        }

        // 3) Average & clusters
        let total: f64 = diffs.values().sum();
        let avg = total / (diffs.len() as f64);
        println!("\n  Average Δ for this drug: +{:.2}", avg);
        let (high, low): (Vec<_>, Vec<_>) =
            diffs.clone().into_iter().partition(|(_, &v)| v >= avg);

        print_cluster("Above average", &high);
        print_cluster("Below average", &low);

        // 4) Generate & save chart
        plot_diff_for_drug(&diffs, &drug)?;
    }

    // After all drugs: overall champion
    println!("\n===== Overall Largest Increase Across All Drugs =====");
    println!("{} with a Δ of +{:.2}", overall_best.0, overall_best.1);

    Ok(())
}

fn plot_diff_for_drug(
    diffs: &HashMap<String, f64>,
    drug: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("charts/{}.png", drug.replace('/', "_"));
    std::fs::create_dir_all("charts")?;
    let root = BitMapBackend::new(&filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut races: Vec<String> = diffs.keys().cloned().collect();
    let mut values: Vec<f64>   = races.iter().map(|r| diffs[r]).collect();

    let max_val = values.iter().cloned().fold(f64::MIN, f64::max).max(0.0);
    let min_val = values.iter().cloned().fold(f64::MAX, f64::min).min(0.0);

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Δ Rate since 2010: {}", drug), ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(races.clone().into_segmented(), min_val..max_val)?;

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(10)
        .y_desc("Δ Rate per 100,000")
        .x_desc("Race/Ethnicity")
        .label_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        races.iter().zip(values.iter()).map(|(race, &v)| {
            let x0 = SegmentValue::Exact(race.clone());
            Rectangle::new([(x0, 0.0), (x0.next(), v)], ORANGE.filled())
        }),
    )?;
    root.present()?;
    println!("  ▶ Chart saved to {}", filename);
    Ok(())
}

fn print_cluster(title: &str, bucket: &[(String, f64)]) {
    print!("  {}:", title);
    if bucket.is_empty() {
        println!(" (none)");
    } else {
        println!();
        for (race, Δ) in bucket {
            println!("    • {:<20} +{:.2}", race, Δ);
        }
    }
}