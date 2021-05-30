# IPD Tournament with WebAssembly

NOTE: Currently only the source ABI is present. The actual runner is yet to be implemented.
This is an Iterated Prisoner's Dilemma tournament with WebAssembly players.
You can use a wide variety of languages such as C, Rust, and everything that can compile to WebAssembly.
Templates for Rust and C are provided.

## Howto: Compile

```sh
# substitute CHANGEME with your source code
clang -target wasm32 -nostdlib -Wl,--no-entry -Wl,--export-all -Wl,-z,stack-size=$[8 * 1024 * 1024] -Os -v -o CHANGEME.wasm CHANGEME.c
rustc --target=wasm32-unknown-unknown --crate-type cdylib -o CHANGEME.wasm CHANGEME.rs 
```
