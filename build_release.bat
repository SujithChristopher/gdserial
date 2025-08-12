@echo off
echo Building GdSerial for release...

REM Detect architecture
for /f "delims=" %%a in ('powershell -command "[System.Environment]::Is64BitOperatingSystem"') do set IS_64BIT=%%a
if "%IS_64BIT%"=="True" (
    set ARCH_NAME=x86_64
) else (
    set ARCH_NAME=x86
)

echo Building for platform: windows, architecture: %ARCH_NAME%

REM Build the Rust library
cargo build --release

REM Create platform-specific bin directory
if not exist "addons\gdserial\bin\windows-%ARCH_NAME%" mkdir "addons\gdserial\bin\windows-%ARCH_NAME%"

REM Copy the built library to the platform-specific directory
copy "target\release\gdserial.dll" "addons\gdserial\bin\windows-%ARCH_NAME%\" >nul 2>&1

echo Windows library copied to addons/gdserial/bin/windows-%ARCH_NAME%/
echo Build complete! Library files copied to addons/gdserial/bin/
echo.
echo To publish to Godot Asset Library:
echo 1. Test the addon in a Godot project
echo 2. Create a release on GitHub with the addons/ folder
echo 3. Submit to Godot Asset Library with the release URL