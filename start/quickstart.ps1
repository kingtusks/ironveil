$projectPath = Join-Path (Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)) "ironveil"
$bin = if ($args[0]) { $args[0] } else { "both" }

if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Host "not running as admin, relaunching"
    Start-Process powershell.exe -ArgumentList "-NoExit -ExecutionPolicy Bypass -File `"$PSCommandPath`" $bin" -Verb RunAs
    exit
}

Write-Host "running as admin" -ForegroundColor Green
Set-Location $projectPath

if ($bin -eq "both") {
    Write-Host "starting server and client in separate terminals..." -ForegroundColor Cyan
    Start-Process powershell.exe -ArgumentList "-NoExit -ExecutionPolicy Bypass -Command `"Set-Location '$projectPath'; cargo run --bin server`""
    Start-Sleep -Seconds 2 
    Start-Process powershell.exe -ArgumentList "-NoExit -ExecutionPolicy Bypass -Command `"Set-Location '$projectPath'; cargo run --bin client`""
} else {
    cargo run --bin $bin
}
