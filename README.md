# Multiplatform Serde NBT

### Overview
`mm_io` is a Rust crate designed by Minecraft Manipulator to provide basic I/O functionalities for reading and writing binary or NBT (Named Binary Tag) files. This crate is particularly useful for handling Minecraft-related data files efficiently.

### Features
- Read and write binary files.
- Read and write NBT files.
- Simple and intuitive API.

### Usage
Add the following line to your `Cargo.toml` file:
```toml
[dependencies]
mm_io = "0.4.0"
```

<!--#### Reading NBT files
```rust
// Read u8 from NBT file:
use bin::TagIo;

let x = &vec![0x01];
let mut fr = bin::FileReaderBE::new(x, 0);
let byte = fr.read::<u8>().unwrap();
assert_eq!(byte, 0x01);
```

#### Writing NBT files
```rust

```-->

### Contributing
Contributions are welcome! If you encounter any issues, have feature requests, or want to contribute improvements, feel free to open an issue or submit a pull request.
1. Fork this git repository
2. Clone the git repository you forked
3. Commit your changes
4. Setup a pull request from your fork to the main repository

### License
This crate is dual-licensed under the terms of the [MIT](https://github.com/Minecraft-Manipulator/mm-io/blob/main/LICENSE-MIT) and [Apache](https://github.com/Minecraft-Manipulator/mm-io/blob/main/LICENSE-APACHE) licenses. See the respective license files for more details.
