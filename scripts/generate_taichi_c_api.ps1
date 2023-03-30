$TAICHI_REPO_DIR = $env:TAICHI_REPO_DIR;

if (-not(Test-Path $TAICHI_REPO_DIR)) {
    Write-Host "TAICHI_REPO_DIR is not set"
    exit -1
}

Copy-Item "scripts/generate_rust_language_binding.py" "$TAICHI_REPO_DIR/misc/generate_rust_language_binding.py"
Push-Location $TAICHI_REPO_DIR
& python3 ./misc/generate_rust_language_binding.py
Pop-Location
Copy-Item $TAICHI_REPO_DIR/c_api/rust/*.rs "taichi-sys/src"
