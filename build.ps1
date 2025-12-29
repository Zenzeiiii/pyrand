# BUILD.PS1 - Build wheel without venv, copy, and auto-install

$ErrorActionPreference = "Stop"

# Paths
$projectRoot = Split-Path -Parent $MyInvocation.MyCommand.Definition
$distDir = Join-Path $projectRoot "dist"

# Make dist directory if missing
if (-Not (Test-Path $distDir)) {
    New-Item -ItemType Directory -Path $distDir | Out-Null
}

# Use forward compatibility for PyO3
$env:PYO3_USE_PYTHON_SYS_EXECUTABLE = 1

Write-Host "Building Python module with maturin (wheel build)..."
maturin build --release

Write-Host "Copying wheel(s) to dist/..."
$wheelFiles = Get-ChildItem "$projectRoot\target\wheels\*.whl" -ErrorAction SilentlyContinue
foreach ($file in $wheelFiles) {
    Copy-Item $file.FullName $distDir -Force
}

Write-Host "Installing wheel(s)..."
foreach ($file in $wheelFiles) {
    Write-Host "Installing $($file.Name)..."
    python -m pip install --force-reinstall $file.FullName
}

Write-Host "Build and installation complete. Wheels copied to dist/ and installed."
Write-Host "Running example..."
python .\examples\example.py
