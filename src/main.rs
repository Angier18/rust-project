use rust_project::{analysis, csv_reader};
use rust_project::data_model::OverdoseRecord;
use plotters::prelude::*;
use plotters::style::full_palette::ORANGE;
use std::collections::HashMap;

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

fn plot_diff_for_drug(
    diffs: &HashMap<String, f64>,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("charts")?;
    let name = label
        .replace(' ', "_")
        .replace(':', "")
        .replace('/', "_");
    let filename = format!("charts/{}.png", name);

    let root = BitMapBackend::new(&filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let races: Vec<String> = diffs.keys().cloned().collect();
    let values: Vec<f64> = races.iter().map(|r| diffs[r]).collect();
    let n = races.len() as i32;

    let max_y = values.iter().cloned().fold(f64::MIN, f64::max).max(0.0);
    let min_y = values.iter().cloned().fold(f64::MAX, f64::min).min(0.0);

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Δ Rate since 2010: {}", label), ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(80)
        .y_label_area_size(60)
        .build_cartesian_2d(0..n, min_y..max_y)?;

    chart
        .configure_mesh()
        .x_labels(n as usize)
        .x_label_formatter(&|x| {
            let idx = *x as usize;
            if idx < races.len() {
                races[idx].clone()
            } else {
                String::new()
            }
        })
        .x_desc("Race/Ethnicity")
        .y_desc("Δ Rate per 100,000")
        .label_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series((0..n).map(|i| {
        Rectangle::new(
            [(i, 0.0), (i + 1, values[i as usize])],
            ORANGE.filled(),
        )
    }))?;

    root.present()?;
    println!("  ▶ Chart saved to {}", filename);
    Ok(())
}

fn run_analysis_and_plot(
    label: &str,
    subset: &[OverdoseRecord],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n==== Analysis for: {} ====", label);
    if subset.is_empty() {
        println!("  No records for this category.");
        return Ok(());
    }

    let diffs = analysis::increase_since_2010(subset);
    if diffs.is_empty() {
        println!("  Not enough years to compute a Δ.");
        return Ok(());
    }

    let mut sorted: Vec<(&String, &f64)> = diffs.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    println!("  Race/Ethnicity                 Δ Rate since 2010");
    println!("  ----------------------------------------------");
    for (race, delta) in &sorted {
        println!("  {:<30} {:+.2}", race, *delta);
    }

    if let Some(&(race, delta)) = sorted.first() {
        println!("\n  🏆 Highest Δ: {} at +{:.2}", race, delta);
    }

    let total: f64 = diffs.values().sum();
    let avg = total / (diffs.len() as f64);
    println!("\n  Average Δ: +{:.2}", avg);

    let (high, low): (Vec<_>, Vec<_>) =
        diffs.clone().into_iter().partition(|(_, v)| *v >= avg);
    print_cluster("Above average", &high);
    print_cluster("Below average", &low);

    plot_diff_for_drug(&diffs, label)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/Drug_overdose_death_rates__by_drug_type__sex__age__race__and_Hispanic_origin__United_States.csv";
    let all = csv_reader::read_records(path)?;

    let demo_only: Vec<OverdoseRecord> = all
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
}