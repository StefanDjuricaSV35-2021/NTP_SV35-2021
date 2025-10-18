use std::thread;
use std::time::{Instant, Duration};
use std::cmp::min;

type Matrix = Vec<Vec<f64>>;

fn get_minor(matrix: &Matrix, row_to_remove: usize, col_to_remove: usize) -> Matrix {
    matrix.iter().enumerate()
        .filter(|(i, _)| *i != row_to_remove)
        .map(|(_, row_val)| {
            row_val.iter().enumerate()
                .filter(|(j, _)| *j != col_to_remove)
                .map(|(_, &val)| val)
                .collect()
        })
        .collect()
}

fn determinant_sequential_worker(matrix: &Matrix) -> f64 {
    let n = matrix.len();

    if n == 1 {
        return matrix[0][0];
    }
    if n == 2 {
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    }

    let mut determinant = 0.0;
    for j in 0..n {
        let sign = (-1.0f64).powi(j as i32);
        let minor = get_minor(matrix, 0, j);
        determinant += sign * matrix[0][j] * determinant_sequential_worker(&minor);
    }
    determinant
}

pub fn determinant_parallel(matrix: &Matrix, num_threads: usize) -> f64 {
    let n = matrix.len();

    // bazni slucaj
    if n <= 2 { 
        return determinant_sequential_worker(matrix);
    }

    // Ne mozemo koristiti vise niti nego što ima minora
    let threads_to_use = min(n, num_threads);
    let mut handles = vec![];

    // kreiramo listu svih minora unapred
    let minors: Vec<(usize, Matrix)> = (0..n)
        .map(|j| (j, get_minor(matrix, 0, j)))
        .collect();

    // Delimo listu minora na delove
    let chunks: Vec<_> = minors.chunks((n as f64 / threads_to_use as f64).ceil() as usize).collect();

    for chunk in chunks {
        // kloniramo podatke koji ce biti premesteni u nit
        let chunk_owned = chunk.to_vec();
        
        let handle = thread::spawn(move || {
            let mut results = Vec::new();
            for (original_j, minor) in chunk_owned {
                let det = determinant_sequential_worker(&minor);
                results.push((original_j, det));
            }
            results
        });
        handles.push(handle);
    }

    let mut minor_determinants: Vec<(usize, f64)> = Vec::new();
    for handle in handles {
        minor_determinants.extend(handle.join().unwrap());
    }

    // sortiramo rezultate po originalnoj poziciji kolone 'j' da bismo sacuvali redosled
    minor_determinants.sort_by_key(|k| k.0);

    let mut determinant = 0.0;
    for (j, minor_det) in minor_determinants {
        let sign = (-1.0f64).powi(j as i32);
        determinant += sign * matrix[0][j] * minor_det;
    }

    determinant
}

fn generate_matrix(size: usize) -> Matrix {
    (0..size).map(|_| (0..size).map(|_| rand::random::<f64>() * 10.0).collect()).collect()
}

pub fn run_parallel_determinant(matrix_size: usize, num_threads: usize) -> Duration { 
    let threads_info = format!("{} niti", num_threads);
    println!(
        "Računanje determinante za matricu {}x{} (paralelno, Rust, koristeći {})...",
        matrix_size, matrix_size, threads_info
    );

    let matrix = generate_matrix(matrix_size);
    
    let start = Instant::now();
    let det = determinant_parallel(&matrix, num_threads);
    let duration = start.elapsed();

    println!("Determinanta: {}", det);
    println!("Vreme izvršavanja: {:.4?}", duration);

    duration
}