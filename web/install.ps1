# cantrip installer — Windows PowerShell and PowerShell 7.
$cantripPreviousError = $ErrorActionPreference
$ErrorActionPreference = 'Stop'
try {
    $cantripEscape = [char]27
    $cantripColor = $Host.UI.SupportsVirtualTerminal -and -not $env:NO_COLOR
    $cantripAqua = if ($cantripColor) { "$cantripEscape[38;2;142;192;124m" } else { '' }
    $cantripOrange = if ($cantripColor) { "$cantripEscape[38;2;254;128;25m" } else { '' }
    $cantripReset = if ($cantripColor) { "$cantripEscape[0m" } else { '' }
    Write-Host "`n  ${cantripOrange}✦${cantripReset} ${cantripAqua}cantrip${cantripReset}"
    Write-Host "  speak plainly. get the incantation.`n"
    if ($env:CANTRIP_INSTALL_DIR) {
        $cantripDirectory = $env:CANTRIP_INSTALL_DIR
        Remove-Item Env:\CANTRIP_CLI_INSTALL_DIR -ErrorAction SilentlyContinue
        Remove-Item Env:\CARGO_DIST_FORCE_INSTALL_DIR -ErrorAction SilentlyContinue
        $env:CANTRIP_CLI_UNMANAGED_INSTALL = $cantripDirectory
    } else {
        $cantripRoot = if ($env:CANTRIP_CLI_INSTALL_DIR) { $env:CANTRIP_CLI_INSTALL_DIR } elseif ($env:CARGO_DIST_FORCE_INSTALL_DIR) { $env:CARGO_DIST_FORCE_INSTALL_DIR } elseif ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $HOME '.cargo' }
        $cantripDirectory = Join-Path $cantripRoot 'bin'
    }
    Write-Host '  Windows · fetching the latest stable incantation…'
    $cantripInstaller = Invoke-RestMethod -Uri 'https://github.com/flockcap/cantrip/releases/latest/download/cantrip-cli-installer.ps1'
    Invoke-Expression $cantripInstaller
    $cantripBinary = Join-Path $cantripDirectory 'cantrip.exe'
    & $cantripBinary --version
    if ($LASTEXITCODE -ne 0) { throw 'cantrip version check failed' }
    Write-Host "`n  ${cantripAqua}✓ ready${cantripReset}"
    Write-Host "  try:    cantrip find the largest files here"
    Write-Host "  update: cantrip update`n`n  by flock · cantrip.fyi`n"
} finally { $ErrorActionPreference = $cantripPreviousError }
