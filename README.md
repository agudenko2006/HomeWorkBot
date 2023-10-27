# HomeWorkBot

## Getting started

```bash
export TELOXIDE_TOKEN=<Your token here>
git clone https://github.com/agudenko2006/homeworkbot
cd homeworkbot
cargo run
```

## Building for a Raspberry Pi

I build is using [cross](https://github.com/cross-rs/cross/wiki/Getting-Started):
```bash
git clone https://github.com/agudenko2006/homeworkbot
cd homeworkbot
cross build --release --target aarch64-unknown-linux-gnu
```
