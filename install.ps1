#!/usr/bin/env pwsh
$Repo = "Nisoku/UPI"

# Platform detection
$Arch = if ([Environment]::Is64BitOperatingSystem) { "amd64" } else { "i386" }
$Platform = "windows-${Arch}"

# Get latest release tag
$Tag = $env:UPI_VERSION
if (-not $Tag) {
  $Release = Invoke-RestMethod -Uri "https://api.github.com/repos/${Repo}/releases/latest"
  $Tag = $Release.tag_name
}
Write-Host "latest release: ${Tag}"

# Determine install dir
$InstallDir = $env:UPI_INSTALL_DIR
if (-not $InstallDir) {
  $LocalBin = Join-Path $env:USERPROFILE ".local\bin"
  if (Test-Path "$env:SystemRoot\System32" -PathType Container) {
    # Check if we can write to a user-local path
    $InstallDir = $LocalBin
  } else {
    $InstallDir = $LocalBin
  }
}
[System.IO.Directory]::CreateDirectory($InstallDir) | Out-Null

# Download & extract
$TarballUrl = "https://github.com/${Repo}/releases/download/${Tag}/upi-${Tag}.tar.gz"
$TarballPath = Join-Path $env:TEMP "upi.tar.gz"
$ExtractPath = Join-Path $env:TEMP "upi-extract"

Write-Host "downloading ${TarballUrl}"
Invoke-WebRequest -Uri $TarballUrl -OutFile $TarballPath

# Extract (PowerShell 5+ has Expand-Archive, but for tar.gz we use tar.exe on Win10+)
if (Get-Command tar.exe -ErrorAction SilentlyContinue) {
  Remove-Item -Path $ExtractPath -Recurse -Force -ErrorAction SilentlyContinue
  New-Item -ItemType Directory -Path $ExtractPath -Force | Out-Null
  tar -xzf $TarballPath -C $ExtractPath
} else {
  # Fallback: .NET 4.5+ GZipStream + Tar (simplified: assume 7zip or manual)
  Write-Warning "tar.exe not found. Install 7-Zip or Windows 10+ build 17063+."
  exit 1
}

$BinaryName = "upi-windows-${Arch}.exe"
$BinaryPath = Join-Path $ExtractPath $BinaryName
$DestPath = Join-Path $InstallDir "upi.exe"

if (-not (Test-Path $BinaryPath)) {
  Write-Error "binary not found in archive: ${BinaryName}"
  Get-ChildItem $ExtractPath | Select-Object Name | Write-Host
  exit 1
}

Copy-Item $BinaryPath $DestPath -Force
Remove-Item $TarballPath -Force -ErrorAction SilentlyContinue
Remove-Item $ExtractPath -Recurse -Force -ErrorAction SilentlyContinue

Write-Host "installed upi to ${DestPath}"

# Check PATH
$InPath = [Environment]::GetEnvironmentVariable("PATH", "User") -split ";" -contains $InstallDir
if (-not $InPath) {
  Write-Warning "${InstallDir} is not in PATH. Add it manually or rerun the script."
}
