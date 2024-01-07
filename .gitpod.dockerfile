from debian as base
run "apt update && apt install build-essential curl libpq-dev -y"
run "curl https://sh.rustup.rs -sSf | sh -s -- -y"
run rustup toolchain add nightly
run rustup target add wasm32-unknown-unknown
run cargo install wasm-opt wasm-bindgen-cli trunk cargo-add
