# fash

`fash` (file hasher) is a cross-platform GUI-app for computing hashes of files and comparing them to supplied hashes. `fash` can compute hashes using the algorithms SHA256, SHA1 and MD5.

<div align="center">
  <img src="assets/demo.gif" alt="Demo GIF of FeO running."/>
  <p>
    <i>Demo GIF where file is dragged and dropped.</i>
  </p>
</div>

Written in Rust, using the library <a href="https://github.com/emilk/egui">egui</a> for the GUI, and libraries from <a href="https://github.com/RustCrypto">RustCrypto</a> for computing hashes. Example code from `egui` is used to create file dialog.


# Installation

`fash` can be installed via <a href="https://rust-lang.org/tools/install">cargo</a>, with

```bash
cargo install fash
```

# License

fash is dual-licensed under either

* MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. 