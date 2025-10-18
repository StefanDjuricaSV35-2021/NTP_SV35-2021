import time
import random
import multiprocessing
import argparse
import os

OUTPUT_FILE = "../results.csv"

def get_minor(matrix, row, col):
    """Vraća podmatricu (minor) bez specificirane vrste i kolone."""
    return [row_val[:col] + row_val[col+1:] for i, row_val in enumerate(matrix) if i != row]

def determinant_sequential_worker(matrix):
    """
    SEKVENCIJALNA funkcija za računanje determinante.
    Ovu funkciju će IZVRŠAVATI RADNICI iz pool-a.
    Ona ne kreira novi pool.
    """
    n = len(matrix)
    if n == 1:
        return matrix[0][0]
    if n == 2:
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]

    determinant = 0
    for j in range(n):
        sign = (-1) ** j
        minor = get_minor(matrix, 0, j)
        determinant += sign * matrix[0][j] * determinant_sequential_worker(minor)
    return determinant

def determinant_parallel(matrix, num_processes=None):
    """
    PARALELNA funkcija koja samo na najvišem nivou raspodeljuje posao.
    """
    n = len(matrix)
    if n == 1:
        return matrix[0][0]
    if n == 2:
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]

    minors = [get_minor(matrix, 0, j) for j in range(n)]

    with multiprocessing.Pool(processes=num_processes) as pool:
        minor_determinants = pool.map(determinant_sequential_worker, minors)

    determinant = 0
    for j in range(n):
        sign = (-1) ** j
        determinant += sign * matrix[0][j] * minor_determinants[j]

    return determinant

def generate_matrix(size):
    """Generiše kvadratnu matricu zadate veličine sa nasumičnim brojevima."""
    return [[random.randint(0, 10) for _ in range(size)] for _ in range(size)]

def main():
    parser = argparse.ArgumentParser(description="Paralelno računanje determinante matrice.")
    parser.add_argument('--size', type=int, required=True, help='Veličina kvadratne matrice.')
    parser.add_argument('--processes', type=int, default=None, help='Broj procesa koji će se koristiti (podrazumevano: broj CPU jezgara).')
    
    args = parser.parse_args()

    matrix_size = args.size
    num_processes = args.processes

    actual_processes = num_processes if num_processes else multiprocessing.cpu_count()
    processes_info = f"{actual_processes} procesa"
    print(f"Računanje determinante za matricu {matrix_size}x{matrix_size} (paralelno, koristeći {processes_info})...")
    
    matrix = generate_matrix(matrix_size)

    start_time = time.time()
    det = determinant_parallel(matrix, num_processes)
    end_time = time.time()

    duration = end_time - start_time

    print(f"Determinanta: {det}")
    print(f"Vreme izvršavanja: {duration:.4f} sekundi")

    header_needed = not os.path.exists(OUTPUT_FILE)
    with open(OUTPUT_FILE, 'a') as f:
        if header_needed:
            f.write("implementation,size,threads,time\n")
        f.write(f"python_par,{matrix_size},{actual_processes},{duration:.4f}\n")
    
    print(f"Rezultat je sačuvan u {OUTPUT_FILE}")


if __name__ == "__main__":
    main()