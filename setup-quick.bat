@echo off
chcp 65001 >nul
echo ========================================
echo py2rs Quick Setup (Debug Build)
echo ========================================
echo.

echo [0/4] Checking icons...
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

echo [1/4] Installing npm dependencies...
cd py2rs
call npm install
if errorlevel 1 (
    echo [ERROR] npm install failed
    pause
    exit /b 1
)

echo.
echo [2/4] Installing Rust dependencies...
cd src-tauri
cargo fetch
if errorlevel 1 (
    echo [ERROR] cargo fetch failed
    pause
    exit /b 1
)

echo.
echo [3/4] Building project (debug mode - faster)...
cargo build
if errorlevel 1 (
    echo [ERROR] Build failed
    echo.
    echo Troubleshooting:
    echo 1. Close any antivirus temporarily
    echo 2. Run as Administrator
    echo 3. Add exception for: %cd%
    pause
    exit /b 1
)

cd ..\..

echo.
echo Setup complete!
echo.
echo To run debug version: npm run tauri dev
echo For release build, run: setup.bat
echo.
pause
