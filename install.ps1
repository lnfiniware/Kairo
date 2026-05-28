# KairoDB Standalone Installer for Windows
# Installs KairoDB globally without requiring Rust or Cargo to be installed.

$ErrorActionPreference = "Stop"

# 1. Determine local source executable
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$SourceExe = Join-Path $ScriptDir "target\release\kairo.exe"

if (-not (Test-Path $SourceExe)) {
    $SourceExe = Join-Path $ScriptDir "target\debug\kairo.exe"
}

if (-not (Test-Path $SourceExe)) {
    $SourceExe = Join-Path $ScriptDir "kairo.exe"
}

if (-not (Test-Path $SourceExe)) {
    Write-Error "Could not find kairo.exe in target\release, target\debug, or project root. Please compile the project first using 'cargo build --release'."
    exit 1
}

# 2. Define global installation directory
$InstallDir = Join-Path $env:USERPROFILE ".kairo\bin"

# 3. Create install directory if missing
if (-not (Test-Path $InstallDir)) {
    Write-Host "Creating installation directory: $InstallDir"
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

# 4. Copy the executable
$TargetExe = Join-Path $InstallDir "kairo.exe"
Write-Host "Installing KairoDB executable..."
Copy-Item -Path $SourceExe -Destination $TargetExe -Force

# 5. Also copy to .cargo/bin if it exists (highly likely for user path compatibility)
$CargoBinDir = Join-Path $env:USERPROFILE ".cargo\bin"
if (Test-Path $CargoBinDir) {
    Write-Host "Updating global cargo-bin copy..."
    Copy-Item -Path $SourceExe -Destination (Join-Path $CargoBinDir "kairo.exe") -Force
}

# 6. Add installation directory to User PATH if not already present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$CleanUserPath = $UserPath -split ";" | ForEach-Object { $_.Trim() }

if ($CleanUserPath -notcontains $InstallDir) {
    Write-Host "Adding $InstallDir to user environment PATH..."
    $NewPath = $UserPath
    if (-not $NewPath.EndsWith(";")) {
        $NewPath += ";"
    }
    $NewPath += $InstallDir
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
}

Write-Host "`nSUCCESS: KairoDB has been successfully installed globally!"
Write-Host "Installation path: $TargetExe"
Write-Host "Please restart your VS Code terminal or command prompt for the PATH changes to take effect."
