echo "Installing Rustup..."
curl -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
echo "Installing wasm-pack..."
cargo install wasm-pack
echo "Installing rsw..."
cargo install rsw