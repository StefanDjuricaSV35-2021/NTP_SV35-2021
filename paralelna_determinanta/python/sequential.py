import time
import random
import argparse
import os

OUTPUT_FILE = "../results.csv"

def get_minor(matrix, row, col):
    """Vraća podmatricu (minor) bez specificirane vrste i kolone."""
    return [row_val[:col] + row_val[col+1:] for i, row_val in enumerate(matrix) if i != row]

def determinant_sequential(matrix):
    """Rekurzivno računa determinantu matrice."""
    n = len(matrix)

    if n == 1:
        return matrix[0][0]
    if n == 2:
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]

    determinant = 0
    for j in range(n):
        sign = (-1) ** j
        minor = get_minor(matrix, 0, j)
        determinant += sign * matrix[0][j] * determinant_sequential(minor)
    return determinant

def generate_matrix(size):
    """Generiše kvadratnu matricu zadate veličine sa nasumičnim brojevima."""
    return [[random.randint(0, 10) for _ in range(size)] for _ in range(size)]

def main():
    parser = argparse.ArgumentParser(description="Sekvencijalno računanje determinante matrice.")
    parser.add_argument('--size', type=int, required=True, help='Veličina kvadratne matrice.')
    
    args = parser.parse_args()
    matrix_size = args.size

    print(f"Računanje determinante za matricu {matrix_size}x{matrix_size} (sekvencijalno)...")

    matrix = generate_matrix(matrix_size)

    start_time = time.time()
    det = determinant_sequential(matrix)
    end_time = time.time()

    duration = end_time - start_time

    print(f"Determinanta: {det}")
    print(f"Vreme izvršavanja: {duration:.4f} sekundi")

    header_needed = not os.path.exists(OUTPUT_FILE)
    with open(OUTPUT_FILE, 'a') as f:
        if header_needed:
            f.write("implementation,size,threads,time\n")
        f.write(f"python_seq,{matrix_size},1,{duration:.4f}\n")
    
    print(f"Rezultat je sačuvan u {OUTPUT_FILE}")

if __name__ == "__main__":
    main()