# Windows build script
# Run with: powershell -ExecutionPolicy Bypass -File scripts/build/windows.ps1

$ErrorActionPreference = "Stop"

Write-Host "Building Cleanux for Windows..."

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location "$ScriptDir/../.."

cargo tauri build --target x86_64-pc-windows-msvc

Write-Host "Windows build complete. Packages available in:"
Write-Host "  src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/"
Write-Host "  src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/"
