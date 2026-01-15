<#
.SYNOPSIS
    KON - Demo runner script for Rust projects

.DESCRIPTION
    Build and run demo projects with various options

.PARAMETER Demo
    The demo to run (e.g., "app_demo" or "ecs_demo/query_demo")

.PARAMETER Release
    Build and run in release mode

.PARAMETER Trace
    Set log level to 'trace'

.PARAMETER Quiet
    Set log level to 'error'

.PARAMETER List
    List all available demos

.EXAMPLE
    ./kon.ps1 app_demo

.EXAMPLE
    ./kon.ps1 ecs_demo/query_demo

.EXAMPLE
    ./kon.ps1 -Release ecs_demo/tag_demo
#>

[CmdletBinding(DefaultParameterSetName = 'Run')]
param(
    [Parameter(Position = 0, ParameterSetName = 'Run')]
    [string]$Demo,

    [Parameter(ParameterSetName = 'Run')]
    [Alias('r')]
    [switch]$Release,

    [Parameter(ParameterSetName = 'Run')]
    [Alias('t')]
    [switch]$Trace,

    [Parameter(ParameterSetName = 'Run')]
    [Alias('q')]
    [switch]$Quiet,

    [Parameter(ParameterSetName = 'List')]
    [Alias('l')]
    [switch]$List,

    [Parameter(ParameterSetName = 'Help')]
    [Alias('h')]
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

# ============================================================================
# Colors & Styling
# ============================================================================
function Write-Colored
{
    param(
        [string]$Text,
        [ConsoleColor]$Color = 'White'
    )
    Write-Host $Text -ForegroundColor $Color -NoNewline
}

function Write-ColoredLine
{
    param(
        [string]$Text,
        [ConsoleColor]$Color = 'White'
    )
    Write-Host $Text -ForegroundColor $Color
}

# ============================================================================
# Banner
# ============================================================================
function Show-Banner
{
    Write-ColoredLine ""
    Write-ColoredLine "  ╦╔═╔═╗╔╗╔" -Color Cyan
    Write-ColoredLine "  ╠╩╗║ ║║║║" -Color Cyan
    Write-ColoredLine "  ╩ ╩╚═╝╝╚╝" -Color Cyan
    Write-ColoredLine ""
}

# ============================================================================
# Help
# ============================================================================
function Show-Help
{
    Show-Banner

    Write-ColoredLine "USAGE" -Color White
    Write-Colored "  ./kon.ps1 " -Color White
    Write-Colored "[OPTIONS] " -Color DarkGray
    Write-ColoredLine "<demo>" -Color Green
    Write-Colored "  ./kon.ps1 " -Color White
    Write-Colored "[OPTIONS] " -Color DarkGray
    Write-ColoredLine "<category>/<demo>" -Color Green
    Write-Host ""

    Write-ColoredLine "OPTIONS" -Color White
    Write-Colored "  -Release, -r    " -Color Yellow
    Write-ColoredLine "Build and run in release mode" -Color White
    Write-Colored "  -Trace, -t      " -Color Yellow
    Write-ColoredLine "Set log level to 'trace'" -Color White
    Write-Colored "  -Quiet, -q      " -Color Yellow
    Write-ColoredLine "Set log level to 'error'" -Color White
    Write-Colored "  -List, -l       " -Color Yellow
    Write-ColoredLine "List all available demos" -Color White
    Write-Colored "  -Help, -h       " -Color Yellow
    Write-ColoredLine "Show this help message" -Color White
    Write-Host ""

    Write-ColoredLine "EXAMPLES" -Color White
    Write-ColoredLine "  # Run app_demo" -Color DarkGray
    Write-ColoredLine "  ./kon.ps1 app_demo" -Color White
    Write-Host ""
    Write-ColoredLine "  # Run specific ecs demo" -Color DarkGray
    Write-ColoredLine "  ./kon.ps1 ecs_demo/query_demo" -Color White
    Write-Host ""
    Write-ColoredLine "  # Run in release mode" -Color DarkGray
    Write-ColoredLine "  ./kon.ps1 -Release ecs_demo/tag_demo" -Color White
    Write-Host ""
}

# ============================================================================
# List Demos
# ============================================================================
function Show-Demos
{
    Show-Banner

    Write-ColoredLine "AVAILABLE DEMOS" -Color White
    Write-Host ""

    $demosPath = "demos"
    if (-not (Test-Path $demosPath))
    {
        Write-ColoredLine "  No demos directory found." -Color DarkGray
        return
    }

    Get-ChildItem -Path $demosPath -Directory | ForEach-Object {
        $category = $_.Name
        $cargoPath = Join-Path $_.FullName "Cargo.toml"

        if (Test-Path $cargoPath)
        {
            $cargoContent = Get-Content $cargoPath -Raw

            # Extract [[bin]] names
            $binMatches = [regex]::Matches($cargoContent, '\[\[bin\]\][^\[]*name\s*=\s*"([^"]+)"')

            if ($binMatches.Count -gt 0)
            {
                Write-Colored "  $category/" -Color Magenta
                Write-Host ""
                foreach ($match in $binMatches)
                {
                    $binName = $match.Groups[1].Value
                    Write-Colored "    • " -Color Green
                    Write-ColoredLine "$category/$binName" -Color White
                }
            } else
            {
                Write-Colored "  • " -Color Green
                Write-ColoredLine "$category" -Color White
            }
        }
    }
    Write-Host ""
}

# ============================================================================
# Error Handling
# ============================================================================
function Write-Error-Message
{
    param([string]$Message)
    Write-Colored "Error: " -Color Red
    Write-ColoredLine $Message -Color White
}

function Write-Warning-Message
{
    param([string]$Message)
    Write-Colored "Warning: " -Color Yellow
    Write-ColoredLine $Message -Color White
}

# ============================================================================
# Main Logic
# ============================================================================

# Handle parameter sets
if ($Help)
{
    Show-Help
    Show-Demos
    exit 0
}

if ($List)
{
    Show-Demos
    exit 0
}

if ([string]::IsNullOrWhiteSpace($Demo))
{
    Show-Help
    Show-Demos
    exit 1
}

# Determine mode and log level
$mode = if ($Release)
{ "release" 
} else
{ "debug" 
}
$logLevel = if ($Trace)
{ "trace" 
} elseif ($Quiet)
{ "error" 
} else
{ "debug" 
}
$cargoFlags = if ($Release)
{ @("--release") 
} else
{ @() 
}

# Parse category/demo format
if ($Demo -match '/')
{
    $parts = $Demo -split '/', 2
    $category = $parts[0]
    $binName = $parts[1]
} else
{
    $category = $Demo
    $binName = $null
}

# Validate demo exists
$demoPath = Join-Path "demos" $category
if (-not (Test-Path $demoPath))
{
    Write-Error-Message "Demo '$category' not found."
    Write-Host ""
    Show-Demos
    exit 1
}

# ============================================================================
# Run
# ============================================================================
$separator = "━" * 60

Write-ColoredLine $separator -Color Cyan
Write-Colored "  Demo:  " -Color White
Write-ColoredLine $Demo -Color Green
Write-Colored "  Mode:  " -Color White
Write-ColoredLine $mode -Color $(if ($Release)
    { 'Yellow' 
    } else
    { 'White' 
    })
Write-Colored "  Log:   " -Color White
Write-ColoredLine $logLevel -Color White
Write-ColoredLine $separator -Color Cyan
Write-Host ""

# Set environment variables
$env:RUST_BACKTRACE = "1"
$env:RUST_LOG = $logLevel

# Build cargo command
$cargoArgs = @("run", "-p", $category)

if ($binName)
{
    $cargoArgs += @("--bin", $binName)
}

$cargoArgs += $cargoFlags

# Execute
try
{
    & cargo @cargoArgs
    if ($LASTEXITCODE -ne 0)
    {
        exit $LASTEXITCODE
    }
} catch
{
    Write-Error-Message $_.Exception.Message
    exit 1
}
