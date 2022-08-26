cargo build --release
mkdir output
mkdir .\output\App
mkdir .\output\App\bin
move .\target\release\discord_presence.exe .\output\App\bin
mkdir .\output\App\assets
xcopy /S .\assets .\output\App\assets