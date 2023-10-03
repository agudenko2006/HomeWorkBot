# HomeWorkBot

## Getting started

```bash
export TELOXIDE_TOKEN=<Your token here>
git clone https://github.com/agudenko2006/homeworkbot
cd homeworkbot
cargo run
```

## Building for a Raspberry Pi

On the desktop:
```bash
git clone https://github.com/agudenko2006/homeworkbot
cd homeworkbot
cargo build --release --target aarch64-unknown-linux-gnu
```