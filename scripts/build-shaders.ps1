<#
.SYNOPSIS
Compiles all shaders embedded by Oxide's D3D12 backend.
.PARAMETER DxcPath
Optional path to dxc.exe. Otherwise searches PATH and installed Windows SDKs.
#>
[CmdletBinding()]
param(
    [string]$DxcPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if (-not $DxcPath) {
    $command = Get-Command dxc.exe -CommandType Application -ErrorAction SilentlyContinue
    if ($command) {
        $DxcPath = $command.Source
    } else {
        $sdkBin = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits\10\bin'
        if (Test-Path -LiteralPath $sdkBin) {
            $versions = Get-ChildItem -LiteralPath $sdkBin -Directory |
                Where-Object { $_.Name -match '^\d+\.\d+\.\d+\.\d+$' } |
                Sort-Object { [version]$_.Name } -Descending
            foreach ($version in $versions) {
                $candidate = Join-Path $version.FullName 'x64\dxc.exe'
                if (Test-Path -LiteralPath $candidate -PathType Leaf) {
                    $DxcPath = $candidate
                    break
                }
            }
        }
    }
}

if (-not $DxcPath -or -not (Test-Path -LiteralPath $DxcPath -PathType Leaf)) {
    throw 'DXC was not found. Install the Windows SDK or add the DXC bin\x64 directory to PATH, or pass -DxcPath C:\path\to\dxc.exe. Keep dxcompiler.dll and dxil.dll with dxc.exe.'
}

$DxcPath = (Resolve-Path -LiteralPath $DxcPath).Path
$shaderDirectory = Join-Path (Split-Path -Parent $PSScriptRoot) 'shaders'
$outputDirectory = Join-Path $shaderDirectory 'compiled'
New-Item -ItemType Directory -Path $outputDirectory -Force | Out-Null

Write-Host "Using DXC: $DxcPath"
foreach ($shader in @('triangle', 'background', 'tonemap', 'shadow', 'camera_surface', 'sprite')) {
    $stages = @('vs', 'ps')
    foreach ($stage in $stages) {
        $source = Join-Path $shaderDirectory "$shader.hlsl"
        $output = Join-Path $outputDirectory "${shader}_${stage}.dxil"
        if (Test-Path -LiteralPath $output) {
            Remove-Item -LiteralPath $output
        }
        & $DxcPath -T "${stage}_6_0" -E "${stage}_main" -Fo $output $source
        if ($LASTEXITCODE -ne 0) {
            throw "DXC failed for ${shader}/${stage} with exit code $LASTEXITCODE."
        }
        if (-not (Test-Path -LiteralPath $output -PathType Leaf) -or
            (Get-Item -LiteralPath $output).Length -eq 0) {
            throw "DXC did not produce bytecode: $output"
        }
        Write-Host "Built $output"
    }
}
