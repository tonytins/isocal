# IsoCal

[![GitHub license](https://img.shields.io/github/license/tonytins/isocal)](https://github.com/tonytins/isocal/blob/main/LICENSE) ![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/tonytins/isocal/Rust/main) ![GitHub commit activity](https://img.shields.io/github/commit-activity/w/tonytins/isocal) [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)

IsoCal is an extension to Chrono's IsoWeek (ISO 8601) that provides additional formatting and functions, such as the "W" prefix to weeks and the ISO ordinal.

## Requirements

- Rust 2021 Edition or newer
- IDEs or Editors
  - [Visual Studio Code](https://code.visualstudio.com/)
  - [InteliJ IDEA](https://www.jetbrains.com/idea/) or [CLion](https://www.jetbrains.com/clion/)

## Installation

```toml
[dependencies]
chrono = "0.4"
isocal = "1"
```

## Example

```rust
use isocal::IsoDate;
use chrono::{Local, IsoWeek, Datelike};

fn main() {
    let now = Local::now();
    let isow = now.iso_week();

    println!("{}", isow.week_fancy());
}
```

## License

This project is dual-licensed under the [BSD-3-Clause](COPYING) or the [UNLICENSE](UNLICENSE).

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.
