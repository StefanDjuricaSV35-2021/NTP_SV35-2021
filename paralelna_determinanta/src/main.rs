use clap::{Parser, ValueEnum};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::thread;

mod sequential;
mod parallel;
mod visualization;

const STRONG_RUST_INPUT: &str = "results_rust.csv";
const STRONG_PYTHON_INPUT: &str = "results_python.csv";
const WEAK_RUST_INPUT: &str = "results_weak_rust.csv";
const WEAK_PYTHON_INPUT: &str = "results_weak_python.csv";
const OUTPUT_FILE: &str = "results.csv";

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    Sequential,
    Parallel,
    Visualize,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, short, value_enum)]
    mode: Mode,
    #[arg(long, short)]
    size: Option<usize>,
    #[arg(long, short)]
    threads: Option<usize>,
}

fn append_result_to_file(record: String) {
    let path = Path::new(OUTPUT_FILE);
    let header_needed = !path.exists() || path.metadata().unwrap().len() == 0;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(OUTPUT_FILE)
        .expect("Nije moguće otvoriti fajl.");

    if header_needed {
        writeln!(file, "implementation,size,threads,time").expect("Nije moguće upisati zaglavlje.");
    }
    writeln!(file, "{}", record).expect("Nije moguće upisati rezultat.");
}

fn print_strong_scaling_table(
    lang: &str,
    size: usize,
    stats: &visualization::PerformanceData,
    speedup: &[(usize, f64)],
) {
    println!("\n\n--- Tabela: Jako Skaliranje ({}) za matricu {}x{} ---", lang, size, size);
    println!("| Broj Jezgara | Srednje Vreme (s) | Std. Devijacija (s) | Ubrzanje (Speedup) |");
    println!("|:--------------:|:-------------------:|:---------------------:|:--------------------:|");

    if let Some((mean, std_dev)) = stats.get(&(size, 1)) {
        println!("| 1              | {:.4}              | {:.4}                | 1.00               |", mean, std_dev);
    }
    
    for (threads, speedup_value) in speedup {
        if *threads > 1 {
            if let Some((mean, std_dev)) = stats.get(&(size, *threads)) {
                 println!("| {}             | {:.4}              | {:.4}                | {:.2}               |", threads, mean, std_dev, speedup_value);
            }
        }
    }
}

fn print_weak_scaling_table(
    lang: &str,
    stats: &visualization::PerformanceData,
    efficiency: &[(usize, f64)],
) {
    println!("\n\n--- Tabela: Slabo Skaliranje ({}) ---", lang);
    println!("| Broj Jezgara | Veličina Matrice | Srednje Vreme (s) | Std. Devijacija (s) | Efikasnost |");
    println!("|:--------------:|:------------------:|:-------------------:|:---------------------:|:------------:|");

    let mut efficiency_map: std::collections::HashMap<_,_> = efficiency.iter().cloned().collect();

    let mut sorted_stats: Vec<_> = stats.iter().collect();
    sorted_stats.sort_by_key(|k| k.0.1);

    for ((size, threads), (mean, std_dev)) in sorted_stats {
        let eff = efficiency_map.remove(threads).unwrap_or(0.0);
        println!("| {}             | {}x{}              | {:.4}              | {:.4}                | {:.2}        |", threads, size, size, mean, std_dev, eff);
    }
}

fn run_visualization() -> Result<(), Box<dyn Error>> {
    println!("--- Pokretanje vizualizacije ---");
    
    // --- 1. Jako skaliranje ---
    println!("\nObrada podataka za jako skaliranje...");
    let rust_records_strong = visualization::read_csv(STRONG_RUST_INPUT)?;
    let python_records_strong = visualization::read_csv(STRONG_PYTHON_INPUT)?;
    
    let rust_seq_records: Vec<_> = rust_records_strong.iter().filter(|r| r.implementation.contains("seq")).cloned().collect();
    let rust_par_records: Vec<_> = rust_records_strong.iter().filter(|r| r.implementation.contains("par")).cloned().collect();
    let python_seq_records: Vec<_> = python_records_strong.iter().filter(|r| r.implementation.contains("seq")).cloned().collect();
    let python_par_records: Vec<_> = python_records_strong.iter().filter(|r| r.implementation.contains("par")).cloned().collect();

    let rust_seq_stats = visualization::calculate_stats(&rust_seq_records);
    let rust_par_stats = visualization::calculate_stats(&rust_par_records);
    let python_seq_stats = visualization::calculate_stats(&python_seq_records);
    let python_par_stats = visualization::calculate_stats(&python_par_records);

    let rust_speedup = visualization::calculate_strong_speedup(&rust_seq_stats, &rust_par_stats);
    let python_speedup = visualization::calculate_strong_speedup(&python_seq_stats, &python_par_stats);
    
    let mut sizes_to_plot: Vec<_> = rust_speedup.keys().cloned().collect();
    sizes_to_plot.sort();

    for size in &sizes_to_plot {
        if let Some(data) = rust_speedup.get(size) {
            visualization::create_strong_scaling_chart(data, *size, "Rust")?;
        }
        if let Some(data) = python_speedup.get(size) {
            visualization::create_strong_scaling_chart(data, *size, "Python")?;
        }
    }

    // --- 2. Slabo skaliranje ---
    println!("\nObrada podataka za slabo skaliranje...");
    let rust_records_weak = visualization::read_csv(WEAK_RUST_INPUT)?;
    let python_records_weak = visualization::read_csv(WEAK_PYTHON_INPUT)?;
    let rust_stats_weak = visualization::calculate_stats(&rust_records_weak);
    let python_stats_weak = visualization::calculate_stats(&python_records_weak);
    let base_key = (8, 1); 
    let rust_efficiency = visualization::calculate_weak_efficiency(&rust_stats_weak, base_key);
    let python_efficiency = visualization::calculate_weak_efficiency(&python_stats_weak, base_key);
    
    visualization::create_weak_scaling_chart(&rust_efficiency, "Rust")?;
    visualization::create_weak_scaling_chart(&python_efficiency, "Python")?;
    
    // --- 3. ISPIS TABELA  ---
    println!("\n--- Generisanje tabela sa rezultatima ---");
    
    let mut full_rust_stats = rust_seq_stats;
    full_rust_stats.extend(rust_par_stats);
    let mut full_python_stats = python_seq_stats;
    full_python_stats.extend(python_par_stats);
    
    for size in &sizes_to_plot {
        print_strong_scaling_table("Rust", *size, &full_rust_stats, rust_speedup.get(size).unwrap());
        print_strong_scaling_table("Python", *size, &full_python_stats, python_speedup.get(size).unwrap());
    }
    
    print_weak_scaling_table("Rust", &rust_stats_weak, &rust_efficiency);
    print_weak_scaling_table("Python", &python_stats_weak, &python_efficiency);

    println!("\n--- Vizualizacija završena! ---");
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Sequential => {
            let size = cli.size.expect("Za sekvencijalni mod, '--size' je obavezan.");
            if cli.threads.is_some() {
                println!("Upozorenje: Argument '--threads' se ignoriše u sekvencijalnom režimu.");
            }
            let duration = sequential::run_sequential_determinant(size);
            let record = format!("rust_seq,{},1,{:.4}", size, duration.as_secs_f64());
            append_result_to_file(record);
            println!("Rezultat je sačuvan u {}", OUTPUT_FILE);
        }
        Mode::Parallel => {
            let size = cli.size.expect("Za paralelni mod, '--size' je obavezan.");
            let num_threads = cli.threads.unwrap_or_else(|| {
                thread::available_parallelism().map_or(1, |n| n.get())
            });
            let duration = parallel::run_parallel_determinant(size, num_threads);
            let record = format!("rust_par,{},{},{:.4}", size, num_threads, duration.as_secs_f64());
            append_result_to_file(record);
            println!("Rezultat je sačuvan u {}", OUTPUT_FILE);
        }
        Mode::Visualize => {
            if let Err(e) = run_visualization() {
                eprintln!("Greška prilikom generisanja grafikona: {}", e);
            }
        }
    }
}