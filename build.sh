cargo build --release --features gui --target x86_64-unknown-linux-gnu
cargo build --release --features gui --target x86_64-pc-windows-gnu
mkdir ./bin 2> /dev/null
cp ./target/x86_64-unknown-linux-gnu/release/rawr ./bin/
cp ./target/x86_64-pc-windows-gnu/release/rawr.exe ./bin/
strip ./bin/*
