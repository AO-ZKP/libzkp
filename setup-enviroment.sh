echo "Setting up enviroment"
echo ""

echo "Checking for rustup:"
rustup --version
echo ""

echo "Checking for cargo:"
cargo --version
echo ""

echo "Adding nightly toolchain:"
rustup toolchain install nightly
echo ""

echo "Adding wasm32-wasip1 target:"
rustup target add wasm32-wasip1 --toolchain nightly
echo ""

echo "Adding wasm32-unknown-emscripten target:"
rustup target add wasm32-unknown-emscripten --toolchain nightly