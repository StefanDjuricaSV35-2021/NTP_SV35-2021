$TestCombinations = @(
    @{ Size = 8;  Threads = 1  }, # Baza: Posao/jezgru ~ 40k
    @{ Size = 9;  Threads = 4  }, # Posao/jezgru ~ 90k (Najbolja opcija za 4 jezgra)
    @{ Size = 9;  Threads = 8  }  # Posao/jezgru ~ 45k 
)
$Repeats = 30
Write-Host "Pokretanje eksperimenata slabog skaliranja..."

# =============================================================
#                         PYTHON TESTOVI
# =============================================================
Write-Host "`n--- Pokretanje Python testova (Slabo skaliranje) ---"
foreach ($combo in $TestCombinations) {
    $size = $combo.Size
    $threads = $combo.Threads
    Write-Host "Test: Python, Size: $size, Threads: $threads, Repeats: $Repeats"
    for ($i = 0; $i -lt $Repeats; $i++) {
        python python/parallel.py --size $size --processes $threads
    }
}

# =============================================================
#                          RUST TESTOVI
# =============================================================
Write-Host "`n--- Pokretanje Rust testova (Slabo skaliranje) ---"
foreach ($combo in $TestCombinations) {
    $size = $combo.Size
    $threads = $combo.Threads
    Write-Host "Test: Rust, Size: $size, Threads: $threads, Repeats: $Repeats"
    for ($i = 0; $i -lt $Repeats; $i++) {
        cargo run --release -- --mode parallel --size $size --threads $threads
    }
}

Write-Host "`nSvi eksperimenti su završeni!"