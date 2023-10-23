# rust-on-m5stack

Rust examples for M5Stack.
To implement and flash firmware for M5Stack in Rust, please check following tools at first.

I use [M5Stack GRAY](https://docs.m5stack.com/en/core/gray) to run firmware, maybe you need to modify something to run correctly if you want to run it on other devices.

## espup

This firmware is witten in Rust. You need to install `espup`. first. Please follow this document https://github.com/esp-rs/rust-build .

## espflash

You can write firmware by using [espflash](https://github.com/esp-rs/espflash).

```sh
$ cargo build
$ espflash flash /target/xtensa-esp-espidf/debug/your-project-name
```

## Examples

- [Display](./display/)
- [Button](./button/)
- [SDCard](./sdcard/)
- [IMU](./imu/)
- [Speaker](./speaker/)
- [Microphone](./microphone/)
