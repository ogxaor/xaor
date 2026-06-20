# build-wasm.ps1
# Script to build the WebAssembly package for Xaor

$WasmPackInstalled = Get-Command wasm-pack -ErrorAction SilentlyContinue

if (-not $WasmPackInstalled) {
    Write-Host "wasm-pack is not installed or not in PATH. Trying to check cargo bin..." -ForegroundColor Yellow
    $CargoBinWasmPack = Join-Path $env:USERPROFILE ".cargo\bin\wasm-pack.exe"
    if (Test-Path $CargoBinWasmPack) {
        $WasmPackPath = $CargoBinWasmPack
    } else {
        Write-Error "wasm-pack could not be found. Please wait for cargo install wasm-pack to finish, or install wasm-pack manually."
        exit 1
    }
} else {
    $WasmPackPath = "wasm-pack"
}

Write-Host "Building WebAssembly package using wasm-pack..." -ForegroundColor Green
& $WasmPackPath build --target web --out-dir pkg

if ($LASTEXITCODE -eq 0) {
    Write-Host "WebAssembly package successfully built in 'pkg/' directory!" -ForegroundColor Green
} else {
    Write-Error "Failed to build WebAssembly package."
    exit 1
}
