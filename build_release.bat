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

REM ========================================
REM Copy addon to example folder
REM ========================================
set SCRIPT_DIR=%~dp0
set SOURCE_ADDON=%SCRIPT_DIR%addons\gdserial
set DEST_ADDON=%SCRIPT_DIR%example\addons\gdserial

echo ========================================
echo Copying addon to example project...
echo ========================================
echo.

REM Check if source exists
if not exist "%SOURCE_ADDON%" (
    echo ✗ Error: Source addon not found at %SOURCE_ADDON%
    exit /b 1
)

REM Remove old destination if it exists
if exist "%DEST_ADDON%" (
    echo Removing old addon copy...
    rmdir /s /q "%DEST_ADDON%"
)

REM Create destination directory
mkdir "%DEST_ADDON%"

REM Copy entire addon folder using xcopy
xcopy "%SOURCE_ADDON%" "%DEST_ADDON%" /E /I /H /Y >nul 2>&1

if %ERRORLEVEL% EQU 0 (
    echo ✓ Addon copied successfully!
    echo You can now open the example project in Godot
) else (
    echo ✗ Error: Failed to copy addon
    exit /b 1
)