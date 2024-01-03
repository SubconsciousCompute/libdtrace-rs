Set-Location -Path .\target\dtrace\

$externalToolsPath = ".\releng\external"
$externalToolsDownloaded = Test-Path $externalToolsPath

if (-not $externalToolsDownloaded) {
    & .\releng\Get-ExternalTools.ps1
}

Set-ExecutionPolicy RemoteSigned -Scope Process
& 'C:\Program Files\Microsoft Visual Studio\2022\Community\MSBuild\Current\Bin\MSBuild.exe' .\opendtrace.sln /t:dtrace_dll:Rebuild /p:Configuration=Release /p:Platform=x64