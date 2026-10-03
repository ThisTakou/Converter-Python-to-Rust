@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

echo ========================================
echo py2rs Setup
echo ========================================
echo.

echo [0/5] Checking icons...
if not exist "py2rs\src-tauri\icons\icon.ico" (
    echo Icons not found, creating placeholders...
    call create_icons.bat
    if errorlevel 1 (
        echo [WARNING] Could not create icons, continuing anyway...
    )
) else (
    echo Recreating icons to ensure they are valid...
    call create_icons.bat
)

echo.
where cargo >nul 2>&1
if errorlevel 1 (
    echo [ERROR] Rust not found.
    echo Please install from: https://rustup.rs/
    pause
    exit /b 1
)

where npm >nul 2>&1
if errorlevel 1 (
    echo [ERROR] Node.js not found.
    echo Please install from: https://nodejs.org/
    pause
    exit /b 1
)

echo [1/5] Installing npm dependencies...
cd py2rs
call npm install
if errorlevel 1 (
    echo [ERROR] npm install failed
    pause
    exit /b 1
)

echo.
echo [2/5] Installing Rust dependencies...
cd src-tauri
cargo fetch
if errorlevel 1 (
    echo [ERROR] cargo fetch failed
    pause
    exit /b 1
)

echo.
echo [3/5] Building project...
echo This may take a few minutes...
echo If antivirus blocks the build, add an exception for this folder.
echo.
cargo build --release
if errorlevel 1 (
    echo [ERROR] Build failed
    echo.
    echo Troubleshooting:
    echo 1. Run this script as Administrator
    echo 2. Add exception in antivirus for: %cd%
    echo 3. Close any IDE/editor that may lock files
    echo 4. Try: cargo clean then run setup again
    pause
    exit /b 1
)

cd ..\..

echo.
echo [4/5] Setup complete!
echo.
echo To run: start.bat
echo.
pause
