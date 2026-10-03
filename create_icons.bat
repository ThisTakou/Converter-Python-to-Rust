@echo off
chcp 65001 >nul
echo Generating icons...
echo.

where python >nul 2>&1
if errorlevel 1 (
    echo [WARNING] Python not found. Using fallback method...
    echo.
    goto fallback
)

python create_icons.py
if not errorlevel 1 (
    echo.
    echo Icons created successfully!
    pause
    exit /b 0
)

:fallback
echo Creating minimal icon structure...
echo This is a placeholder > py2rs\src-tauri\icons\icon.ico
echo This is a placeholder > py2rs\src-tauri\icons\icon.png
echo This is a placeholder > py2rs\src-tauri\icons\32x32.png
echo This is a placeholder > py2rs\src-tauri\icons\128x128.png
echo.
echo [WARNING] Placeholder icons created.
echo For proper icons, install Python and Pillow, then run:
echo     python create_icons.py
echo.
pause
