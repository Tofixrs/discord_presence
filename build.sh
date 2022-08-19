cargo build --release
cargo build --release --target=x86_64-pc-windows-gnu
mkdir temp
mkdir outupt
cd temp
mkdir -p App/bin
cp -r ../assets ./App
cp ../target/release/discord_presence ./App/bin/discord_presence
zip -r Linux-portable.zip App
mv Linux-portable.zip ../outupt
rm App/bin/discord_presence
cp ../target/x86_64-pc-windows-gnu/release/discord_presence.exe ./App/bin/discord_presence
zip -r Windows-portable.zip App
mv Windows-portable.zip ../outupt
rm -r App
mkdir discord_presence.AppDir
cd discord_presence.AppDir
mkdir -p ./usr/bin/bin
cp ../../target/release/discord_presence ./usr/bin/bin
echo "#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
EXEC="${HERE}/usr/bin/Bin/discord_presence"
exec "${EXEC}"" >> AppRun
echo "[Desktop Entry]
Name=Discord Presence
Exec=Bin/discord_presence
Type=Application
Categories=Utility
Icon=Icon" >> discord_presence.desktop
wget https://cdn.discordapp.com/avatars/436947586788884490/8ac19ae7b91f0eb090d1ddc8e69c94e0.png?size=256
mv 8ac19ae7b91f0eb090d1ddc8e69c94e0.png?size=256 Icon.png
cp -r ../../assets ./
cd ../
ARCH=x86_64 appimagetool discord_presence.AppDir
mv Discord_Presence-x86_64.AppImage ../outupt/Discord_Presence-linux.AppImage
rm -r discord_presence.AppDir
cd ../
 rm -r temp