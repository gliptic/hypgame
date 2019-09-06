cargo run --bin rustweb-bundler ./rustweb/src/lib.hyp ^
.\rustweb\test\rustweb.js ^
.\rustweb\pkg\rustweb.min.js ^
.\rustweb\pkg\rustweb.zip

REM --package rustweb-code --lib

REM cargo build --package rustweb --lib --release --target wasm32-unknown-unknown && ^
REM cargo run --release --bin rustweb-bundler -- ^
REM ./target/wasm32-unknown-unknown/release/rustweb.wasm ^
REM .\rustweb\pkg\rustweb.js ^
REM .\rustweb\pkg\rustweb.min.js

