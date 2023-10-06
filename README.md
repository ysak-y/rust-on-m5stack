# rust-on-m5stack

Rust examples for M5Stack.
To implement and flash firmware for M5Stack in Rust, please check following tools at first.

## espup

This firmware is witten in Rust. You need to install `espup`. first. Please follow this document https://github.com/esp-rs/rust-build .

## espflash

You can write firmware by using [espflash](https://github.com/esp-rs/espflash).

```sh
$ cargo build
$ espflash flash /target/xtensa-esp-espidf/debug/your-project-name
```