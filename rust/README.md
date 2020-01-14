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
# about template
near call sayhi_bl list_template "" --accountId=marco_bl --homeDir=./
near call sayhi_bl create_template "{\"name\": \"invitation\", \"content\": \"This is invitaion content.\", \"duration\": 100}" --accountId=marco_bl --homeDir=./

# about card
near call sayhi_bl create_card "{\"template_id\": \"default\", \"card_type\": 0, \"public_message\": \"This is public msg content.\", \"private_message\": \"This is private_message conent.\", \"name\": \"invitation\", \"count\": 10, \"total\": 10, \"duration\": 100, \"specify_account\": \"\"}" --accountId=marco_bl --homeDir=./
near call sayhi_bl list_card "" --accountId=marco_bl --homeDir=./
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