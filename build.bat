@echo off
mkdir bin 2>nul
cargo build --release
if errorlevel 1 goto error

copy /Y target\release\wmnkextract.exe bin\wmnkextract.exe
if errorlevel 1 goto error

copy /Y target\release\wmpartinfo.exe bin\wmpartinfo.exe
if errorlevel 1 goto error

echo Build successful! Binaries placed in bin/
exit /b 0

:error
echo Build failed!
exit /b 1
