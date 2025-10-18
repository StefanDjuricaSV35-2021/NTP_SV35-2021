$Sizes = @(8, 9, 10, 11)
$Threads = @(2, 4, 8, 16)
$RepeatsSequential = 10      
$RepeatsParallel = 5         
$OutputFile = "results.csv"

if (Test-Path $OutputFile) {
    Remove-Item $OutputFile
    Write-Host "Stari fajl '$OutputFile' je obrisan."
}
Write-Host "Pokretanje eksperimenata... Ovo može potrajati."

# =============================================================
#                         PYTHON TESTOVI
# =============================================================
Write-Host "`n--- Pokretanje Python testova ---"
foreach ($size in $Sizes) {
    Write-Host "Test: Python Sequential, Size: $size, Repeats: $RepeatsSequential"
    for ($i = 0; $i -lt $RepeatsSequential; $i++) {
        python python/sequential.py --size $size
    }

    foreach ($thread in $Threads) {
        Write-Host "Test: Python Parallel, Size: $size, Threads: $thread, Repeats: $RepeatsParallel"
        for ($i = 0; $i -lt $RepeatsParallel; $i++) {
            python python/parallel.py --size $size --processes $thread
        }
    }
}

# =============================================================
#                          RUST TESTOVI
# =============================================================
Write-Host "`n--- Pokretanje Rust testova ---"
foreach ($size in $Sizes) {
    Write-Host "Test: Rust Sequential, Size: $size, Repeats: $RepeatsSequential"
    for ($i = 0; $i -lt $RepeatsSequential; $i++) {
        cargo run --release -- --mode sequential --size $size
    }

    foreach ($thread in $Threads) {
        Write-Host "Test: Rust Parallel, Size: $size, Threads: $thread, Repeats: $RepeatsParallel"
        for ($i = 0; $i -lt $RepeatsParallel; $i++) {
            cargo run --release -- --mode parallel --size $size --threads $thread
        }
    }
}

Write-Host "`nSvi eksperimenti su završeni! Rezultati su sačuvani u '$OutputFile'."