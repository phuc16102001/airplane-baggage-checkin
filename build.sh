rm -rf neardev
cargo build --target wasm32-unknown-unknown --release
near dev-deploy --accountId 'phuc16102001.testnet' --wasmFile 'target\wasm32-unknown-unknown\release\airplane_baggage_checkin.wasm' 