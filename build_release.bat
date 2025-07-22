@echo off
echo Building GdSerial for release...

REM Build the Rust library
cargo build --release

REM Create bin directory if it doesn't exist
if not exist "addons\gdserial\bin" mkdir "addons\gdserial\bin"

REM Copy the built library to the addon directory
copy "target\release\gdserial.dll" "addons\gdserial\bin\" >nul 2>&1
copy "target\release\libgdserial.so" "addons\gdserial\bin\" >nul 2>&1
copy "target\release\libgdserial.dylib" "addons\gdserial\bin\" >nul 2>&1

echo Build complete! Library files copied to addons/gdserial/bin/
echo.
echo To publish to Godot Asset Library:
echo 1. Test the addon in a Godot project
echo 2. Create a release on GitHub with the addons/ folder
echo 3. Submit to Godot Asset Library with the release URL