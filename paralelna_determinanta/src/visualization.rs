use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use plotters::prelude::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    pub implementation: String,
    pub size: usize,
    pub threads: usize,
    pub time: f64,
}

pub fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    Ok(reader.deserialize().collect::<Result<Vec<Record>, _>>()?)
}

pub type PerformanceKey = (usize, usize);
pub type PerformanceStats = (f64, f64); 
pub type PerformanceData = HashMap<PerformanceKey, PerformanceStats>;

pub fn calculate_stats(records: &[Record]) -> PerformanceData {
    let mut times: HashMap<PerformanceKey, Vec<f64>> = HashMap::new();
    for r in records {
        times.entry((r.size, r.threads)).or_default().push(r.time);
    }
    
    times.into_iter().map(|(key, times_vec)| {
        let n = times_vec.len() as f64;
        let mean = times_vec.iter().sum::<f64>() / n;
        let std_dev = (times_vec.iter().map(|time| (time - mean).powi(2)).sum::<f64>() / n).sqrt();
        (key, (mean, std_dev))
    }).collect()
}

pub fn calculate_strong_speedup(
    seq_data: &PerformanceData,
    par_data: &PerformanceData,
) -> HashMap<usize, Vec<(usize, f64)>> {
    let mut speedup_by_size: HashMap<usize, Vec<(usize, f64)>> = HashMap::new();
    for ((size, threads), (par_time, _)) in par_data {
        if let Some((seq_time, _)) = seq_data.get(&(*size, 1)) {
            if *par_time > 0.0 {
                let speedup_value = seq_time / par_time;
                speedup_by_size.entry(*size).or_default().push((*threads, speedup_value));
            }
        }
    }
    for speedups in speedup_by_size.values_mut() {
        speedups.sort_by_key(|a| a.0);
    }
    speedup_by_size
}

fn amdahl_speedup(f: f64, p: usize) -> f64 {
    1.0 / ((1.0 - f) + (f / p as f64))
}

pub fn calculate_weak_efficiency(
    data: &PerformanceData, 
    base_key: PerformanceKey
) -> Vec<(usize, f64)> {
    let base_time = match data.get(&base_key) {
        Some((time, _)) => *time,
        None => return vec![],
    };

    let mut efficiency_data: Vec<(usize, f64)> = data.iter()
        .filter_map(|((_, threads), (avg_time, _))| {
            if *avg_time > 0.0 {
                let efficiency = base_time / avg_time;
                Some((*threads, efficiency))
            } else {
                None
            }
        })
        .collect();
    
    efficiency_data.sort_by_key(|a| a.0);
    efficiency_data
}

pub fn create_strong_scaling_chart(
    speedup_data: &[(usize, f64)],
    matrix_size: usize,
    lang: &str, // "Rust" ili "Python"
) -> Result<(), Box<dyn Error>> {
    let color = if lang == "Rust" { &RED } else { &BLUE };
    let file_name = format!("charts/strong_scaling_{}_size_{}.png", lang.to_lowercase(), matrix_size);
    let root = BitMapBackend::new(&file_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_threads = speedup_data.iter().map(|(t, _)| *t).max().unwrap_or(1);
    let max_speedup_observed = speedup_data.iter().map(|(_, s)| *s).fold(0.0, f64::max);
    let max_y = (max_threads as f64).max(max_speedup_observed).ceil();

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Jako skaliranje ({}) za matricu {}x{}", lang, matrix_size, matrix_size), ("sans-serif", 40).into_font())
        .x_label_area_size(50).y_label_area_size(50).build_cartesian_2d(0..max_threads, 0.0..max_y)?;
    chart.configure_mesh().x_desc("Broj jezgara").y_desc("Ubrzanje (Speedup)").draw()?;
        
    let parallel_fraction = 0.99;
    chart.draw_series(LineSeries::new((1..=max_threads).map(|p| (p, amdahl_speedup(parallel_fraction, p))), &BLACK))?
        .label("Teorijsko ubrzanje (Amdahl, f=0.99)").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart.draw_series(LineSeries::new(speedup_data.iter().cloned(), color))?
        .label(lang).legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).border_style(&BLACK).draw()?;
    root.present()?;
    println!("Grafikon sačuvan u: {}", file_name);
    Ok(())
}

pub fn create_weak_scaling_chart(
    efficiency_data: &[(usize, f64)],
    lang: &str,
) -> Result<(), Box<dyn Error>> {
    let color = if lang == "Rust" { &RED } else { &BLUE };
    let file_name = format!("charts/weak_scaling_{}.png", lang.to_lowercase());
    let root = BitMapBackend::new(&file_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_threads = efficiency_data.iter().map(|(t, _)| *t).max().unwrap_or(1);

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Slabo skaliranje ({})", lang), ("sans-serif", 40).into_font())
        .x_label_area_size(50).y_label_area_size(60).build_cartesian_2d(0..max_threads, 0.0..1.2)?;
    chart.configure_mesh().x_desc("Broj jezgara").y_desc("Efikasnost").draw()?;
        
    chart.draw_series(LineSeries::new([(0, 1.0), (max_threads, 1.0)], &BLACK))?
        .label("Idealna efikasnost (Gustafson)").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart.draw_series(LineSeries::new(efficiency_data.iter().cloned(), color))?
        .label(lang).legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).border_style(&BLACK).draw()?;
    root.present()?;
    println!("Grafikon sačuvan u: {}", file_name);
    Ok(())
}