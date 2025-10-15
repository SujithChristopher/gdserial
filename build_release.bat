@echo off
echo Building GdSerial for release...

REM Build the Rust library
cargo build --release

REM Detect architecture
for /f "tokens=2 delims==" %%a in ('wmic OS get OSArchitecture /value') do set "ARCH=%%a"
if "%ARCH%"=="64-bit" (
    set "ARCH_NAME=x86_64"
) else (
    echo Unsupported architecture: %ARCH%
    exit /b 1
)

echo Building for platform: windows, architecture: %ARCH_NAME%

REM Create platform-specific bin directory
if not exist "addons\gdserial\bin\windows-%ARCH_NAME%" mkdir "addons\gdserial\bin\windows-%ARCH_NAME%"

REM Copy the built library to the platform-specific directory
copy "target\release\gdserial.dll" "addons\gdserial\bin\windows-%ARCH_NAME%\" >nul 2>&1

echo Build complete! Library files copied to addons/gdserial/bin/windows-%ARCH_NAME%/
echo.