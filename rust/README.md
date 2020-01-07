# Status Message

Records the status messages of the accounts that call this contract.

## Testing
To test run:
```bash
cargo test --package status-message -- --nocapture
```

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release


near call bl_card_1 list_template "{\"account_id\": \"aaaa\"}" --accountId=aaaa --homeDir=/Users/rain/Project/near/near-bindgen/examples/status-message/neardev

near deploy --wasmFile=/Users/rain/Project/near/bcvc/rust/target/wasm32-unknown-unknown/release/status_message.wasm --homeDir=/Users/rain/Project/near/bcvc/rust/neardev --contractName=bl_card_1