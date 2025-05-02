use rust_project::{analysis, csv_reader};
use plotters::prelude::*;
use plotters::style::full_palette::ORANGE;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/Drug_overdose_death_rates__by_drug_type__sex__age__race__and_Hispanic_origin__United_States.csv";
    let all = csv_reader::read_records(path)?;
    let mut overall_best: (String, f64) = ("<none>".to_string(), f64::MIN);
    let mut drug_types: Vec<_> = all.iter().map(|r| r.drug_type.clone()).collect();
    drug_types.sort();
    drug_types.dedup();

    for drug in &drug_types {
        println!("\n==== Analysis for: {} ====", drug);
        let subset: Vec<_> = all
            .iter()
            .filter(|r| &r.drug_type == drug)
            .cloned()
            .collect();
        if subset.is_empty() {
            println!("  No records for this category.");
            continue;
        }
        let diffs = analysis::increase_since_2010(&subset);
        if diffs.is_empty() {
            println!("  Not enough years of data to compute a Δ.");
            continue;
        }

        let mut sorted: Vec<(&String, &f64)> = diffs.iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        println!("  Race/Ethnicity                     Δ Rate since 2010");
        println!("  -----------------------------------------------");
        for (race, delta) in &sorted {
            println!("  {:<30} {:+.2}", race, *delta);
        }

        if let Some(&(race, delta)) = sorted.first() {
            println!("\n  🏆 Largest increase: {} at +{:.2}", race, delta);
            if *delta > overall_best.1 {
                overall_best = (race.to_string(), *delta);
            }
        }

        let total: f64 = diffs.values().sum();
        let avg = total / (diffs.len() as f64);
        println!("\n  Average Δ for this drug: +{:.2}", avg);

        let (high, low): (Vec<_>, Vec<_>) =
            diffs.clone().into_iter().partition(|(_, v)| *v >= avg);

        print_cluster("Above average", &high);
        print_cluster("Below average", &low);

        plot_diff_for_drug(&diffs, drug)?;
    }

    println!("\n===== Overall Largest Increase Across All Drugs =====");
    println!("🏅 {} with a Δ of +{:.2}", overall_best.0, overall_best.1);

    Ok(())
}

fn plot_diff_for_drug(
    diffs: &HashMap<String, f64>,
    drug: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("charts")?;
    let filename = format!("charts/{}.png", drug.replace('/', "_"));

    let root = BitMapBackend::new(&filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let races: Vec<String> = diffs.keys().cloned().collect();
    let values: Vec<f64> = races.iter().map(|r| diffs[r]).collect();
    let count = races.len();

    let max_y = values.iter().cloned().fold(f64::MIN, f64::max).max(0.0);
    let min_y = values.iter().cloned().fold(f64::MAX, f64::min).min(0.0);

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Δ Rate since 2010: {}", drug), ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(80)
        .y_label_area_size(60)
        .build_cartesian_2d(0..count, min_y..max_y)?;

    chart
        .configure_mesh()
        .x_labels(count)
        .x_label_formatter(&|idx| races[*idx].clone())
        .x_desc("Race/Ethnicity")
        .y_desc("Δ Rate per 100,000")
        .label_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series((0..count).map(|i| {
        Rectangle::new([(i, 0.0), (i + 1, values[i])], ORANGE.filled())
    }))?;

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
        for (race, delta) in bucket {
            println!("    • {:<20} +{:.2}", race, delta);
        }
    }
}