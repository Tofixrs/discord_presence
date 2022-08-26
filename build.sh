#!/bin/bash
cargo build --release
cargo build --release --target=x86_64-pc-windows-gnu
mkdir temp
mkdir output
cd temp
mkdir -p App/bin
cp -r ../assets ./App
cp ../target/release/discord_presence ./App/bin/discord_presence
zip -r Linux-portable.zip App
mv Linux-portable.zip ../output
rm App/bin/discord_presence
cp ../target/x86_64-pc-windows-gnu/release/discord_presence.exe ./App/bin/discord_presence.exe
zip -r Windows-portable.zip App
mv Windows-portable.zip ../output
rm -r App
mkdir discord_presence.AppDir
cd discord_presence.AppDir
mkdir -p ./usr/bin/bin
cp ../../target/release/discord_presence ./usr/bin/bin
echo "#!/bin/bash
SELF=\$(readlink -f \"\$0\")
HERE=\${SELF%/*}
EXEC=\"\${HERE}/usr/bin/bin/discord_presence\"
exec \"\${EXEC}\"" >> AppRun
chmod +x AppRun
echo "[Desktop Entry]
Name=Discord Presence
Exec=bin/discord_presence
Type=Application
Categories=Utility
Icon=Icon" >> discord_presence.desktop
wget https://cdn.discordapp.com/avatars/436947586788884490/5ec54263e5f8c80ca91674fe5124ffa3.png?size=256
mv 5ec54263e5f8c80ca91674fe5124ffa3.png?size=256 Icon.png
cp -r ../../assets ./usr/bin/
cd ../
ARCH=x86_64 appimagetool discord_presence.AppDir
mv Discord_Presence-x86_64.AppImage ../output/Discord_Presence-linux.AppImage
cd ../
rm -r temp
unzip ./output/Windows-portable.zip