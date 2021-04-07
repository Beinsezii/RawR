cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
mkdir ./bin 2> /dev/null
mkdir ./bin/windows 2> /dev/null
mkdir ./bin/linux 2> /dev/null
for b in rawr rawr_mock_clip rawr_uwu_clip
do
cp ./target/x86_64-unknown-linux-gnu/release/$b ./bin/linux/
cp ./target/x86_64-pc-windows-gnu/release/$b.exe ./bin/windows/
done
strip ./bin/*/*

rm RawR-Windows.zip 2> /dev/null
rm RawR-Linux.tar.zst 2> /dev/null
zip -j RawR-Windows.zip ./bin/windows/*
cd bin/linux
tar -caf ../../RawR-Linux.tar.zst *
cd ../../
