@echo off
chcp 65001 >nul
cd py2rs
call npm run tauri dev
