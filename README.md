# IsoCal

![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/tonytins/isocal/Rust/main)

![GitHub commit activity](https://img.shields.io/github/commit-activity/w/tonytins/isocal)IsoCal is an extension to Chrono's IsoWeek (ISO 8601) that provides additional formatting and functions, such as the "W" prefix to weeks and the ISO ordinal.

## Requirements

- Rust 2021 Edition or newer
- IDEs or Editors
  - [Visual Studio Code](https://code.visualstudio.com/) or [VSCodium](https://vscodium.com/)
  - [InteliJ IDEA Ultimate](https://www.jetbrains.com/idea/) or [RustOver](https://www.jetbrains.com/rust/)

## Installation

```toml
[dependencies]
chrono = "0.4"
isocal = "1"
```

## Usage

```rust
use chrono::prelude::*;
use isocal::IsoDate;

fn main() {
    let now = Local::now();
    let isow = now.iso_week();

    println!("{}", isow.week_fancy());
}
```

## To-do

- [ ] Target Rust 2024 Edition

## License

This project is dual-licensed under the [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).