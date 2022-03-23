# About

This project is try to implement *PFM* generator with *STM32* micro controller and *Rust*

# Pre-requesting
- Necessary to install latest stable *Rust*,

- Add *ARM* microcontroller target:
```bash
rustup target add thumbv7em-none-eabihf
```

- Add Rust(LLVM) version of *binutils*:
```bash
rustup component add llvm-tools-preview
cargo install cargo-binutils
```
- Add other ARM-tooling:
```bash
sudo dnf install openocd
```

# Generate bindings
Download *svd* (System view description) file from *CAD Resource* section,
extract files to `svd/` directory.

# Links
- [Generate Rust register maps (structs) from SVD files](https://crates.io/crates/svd2rust)