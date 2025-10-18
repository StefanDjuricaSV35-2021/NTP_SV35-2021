use std::time::{Instant, Duration};

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

pub fn determinant_sequential(matrix: &Matrix) -> f64 {
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
        determinant += sign * matrix[0][j] * determinant_sequential(&minor);
    }

    determinant
}

fn generate_matrix(size: usize) -> Matrix {
    (0..size).map(|_| (0..size).map(|_| rand::random::<f64>() * 10.0).collect()).collect()
}

pub fn run_sequential_determinant(matrix_size: usize) -> Duration {
    println!(
        "Računanje determinante za matricu {}x{} (sekvencijalno, Rust)...", 
        matrix_size, matrix_size
    );

    let matrix = generate_matrix(matrix_size);

    let start = Instant::now();
    let det = determinant_sequential(&matrix);
    let duration = start.elapsed();

    println!("Determinanta: {}", det);
    println!("Vreme izvršavanja: {:.4?}", duration);

    duration
}