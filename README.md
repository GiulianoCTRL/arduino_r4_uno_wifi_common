# WIP HAL for Arduino UNO r4 WIFI
## Info
This is a  work in progress and currently highly experimental.

## Contributing
The pac code can only be generated with a locked version of svd2rust 0.30.3,
as there are some [issues](https://github.com/rust-embedded/svd2rust/issues/852) with newer versions.

Install svd2rust as follows for working code generation.
```bash
cargo install svd2rust --version 0.30.3 --locked --force
```
