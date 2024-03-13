# Steps to compile `methods.rs`

1. Install the `risc0` toolchain

``` shell
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
```

2. build release

``` shell
cargo build --release
```

The directory of `methods.rs` will be printed to the console, like this 
```shell
warning: methods_path is: "WeatherXM-IoTeX-Textile-POC/zk/target/debug/build/zk-methods-8168d85228fd2c71/out/methods.rs"
```