# Build and package KebiControl
# Made by KebiLab

$ErrorActionPreference = 'Stop'
Set-Location $PSScriptRoot\..

$env:Path = "D:\Projects\VibeCoding\AiProjects\KebiControl\tools\mingw\bin;$env:Path"

Write-Host "Building release..." -ForegroundColor Cyan
cmd /c "set PATH=D:\Projects\VibeCoding\AiProjects\KebiControl\tools\mingw\bin;%PATH% && cargo build --release --bin kebicontrol"
if ($LASTEXITCODE -ne 0) { throw "Build failed" }

$src = "target\x86_64-pc-windows-gnu\release\kebicontrol.exe"
$dst = "dist\KebiControl.exe"
New-Item -ItemType Directory -Force -Path dist | Out-Null
Copy-Item $src $dst -Force

$upx = "tools\upx\upx-4.2.4-win64\upx.exe"
if (Test-Path $upx) {
    Write-Host "Compressing with UPX..." -ForegroundColor Cyan
    & $upx --best --lzma $dst
}

Write-Host "Done. Output: $dst" -ForegroundColor Green
(Get-Item $dst) | Select-Object Name, Length
