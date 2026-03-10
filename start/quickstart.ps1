$projectPath = Join-Path (Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)) "ironveil"
$bin = if ($args[0]) { $args[0] } else { "server" }

if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Host "not running as admin, relaunching"
    Start-Process powershell.exe -ArgumentList "-NoExit -ExecutionPolicy Bypass -File `"$PSCommandPath`" $bin" -Verb RunAs
    exit
}

Write-Host "running as admin" -ForegroundColor Green

Set-Location $projectPath
cargo run --bin $bin

#usage: .\quickstart.ps1 server
#       .\quickstart.ps1 client