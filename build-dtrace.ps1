Set-Location -Path .\target\dtrace\

$externalToolsPath = ".\releng\external"
$externalToolsDownloaded = Test-Path $externalToolsPath

if (-not $externalToolsDownloaded) {
    & .\releng\Get-ExternalTools.ps1
}

Set-ExecutionPolicy RemoteSigned -Scope Process
& msbuild .\opendtrace.sln /t:dtrace_dll:Rebuild /p:Configuration=Release /p:Platform=x64