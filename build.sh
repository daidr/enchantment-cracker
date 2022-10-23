echo "Installing Rustup..."
curl -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
echo "Installing wasm-pack..."
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y
echo "Installing rsw..."
cargo install rsw