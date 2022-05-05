cargo build --target wasm32-unknown-unknown --release
near dev-deploy 'target\wasm32-unknown-unknown\release\airplane_baggage_checkin.wasm'
source 'neardev\dev-account.env'