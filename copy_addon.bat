@echo off
REM Copy addon to example folder after build
REM This script is called by build_release.bat

set SOURCE_ADDON=%~dp0addons\gdserial
set DEST_ADDON=%~dp0example\addons\gdserial

echo.
echo ========================================
echo Copying addon to example project...
echo ========================================

if not exist "%DEST_ADDON%" (
    echo Creating example/addons directory...
    mkdir "%DEST_ADDON%"
)

echo Copying from: %SOURCE_ADDON%
echo Copying to:   %DEST_ADDON%

REM Copy entire addon folder (robocopy mirrors directories)
robocopy "%SOURCE_ADDON%" "%DEST_ADDON%" /E

if %ERRORLEVEL% LEQ 1 (
    echo.
    echo ✓ Addon copied successfully!
    echo You can now open the example project in Godot
) else (
    echo.
    echo ✗ Error copying addon
    exit /b 1
)
