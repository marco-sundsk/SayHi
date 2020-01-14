# SayHi

SayHi backend contract.

## Testing
To test run:
```bash
cargo test --package say-hi -- --nocapture
```
To build:
```bash
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```
-----------
### Call Contract
To init call from terminal:
Let's say, marco_bl is your accountID, and sayhi_bl is the contractID, and the key stored in ./neardev

```bash
near call sayhi_bl list_template "{\"account_id\": \"marco_bl\"}" --accountId=marco_bl --homeDir=./
near call sayhi_bl list_card "{\"account_id\": \"marco_bl\"}" --accountId=marco_bl --homeDir=./
near call sayhi_bl create_template "{\"name\": \"marco_bl\", \"duration\": 100}" --accountId=marco_bl --homeDir=./
near call sayhi_bl create_card "{\"template_id\": \"template_1\", \"public_message\": \"123\", \"private_message\": \"233\", \"name\": \"333\", \"count\": 10, \"is_avg\": true, \"total\": 10, \"duration\": 100}" --accountId=marco_bl --homeDir=./
```

----------
### Deploy Contract
* Prepare contract accountID, let's say, sayhi_bl;
* Send some NEAR to contract account;
* use near shell to deploy;
```bash
cd ts  
near deploy --accountId=sayhi_bl --wasmFile=../rust/target/wasm32-unknown-unknown/release/say_hi.wasm
```

### Call Contract
```bash
near call sayhi_bl create_card "{\"template_id\": \"template_1\"}" --accountId=aaaa
```