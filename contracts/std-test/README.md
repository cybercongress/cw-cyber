# Cyber to Cosmwasm bingings

This is a simple bypassing tester just for testing purpose.

## Running this contract

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via: 

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/std_test.wasm .
ls -l std_test.wasm
sha256sum std_test.wasm
```
