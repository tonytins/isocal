# IsoCal

![Rust](https://github.com/tonytins/isocal/workflows/Rust/badge.svg) [![Build Status](https://travis-ci.org/tonytins/isocal.svg?branch=master)](https://travis-ci.org/tonytins/isocal) [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)

IsoCal is an extension to Chrono's IsoWeek (ISO 8601) that provides additional formatting and functions, such as the "W" prefix to weeks and the ISO ordinal.

## Installation

```toml
[dependencies]
chrono = "0.4"
isocal = "0.1"
```

### Nightly

```toml
[dependencies]
chrono = "0.4"
isocal = { git = "https://github.com/tonytins/isocal" }
```

## Example

```rust
extern crate isocal; // Rust 2015
use isocal::IsoDate;
use chrono::{Local, IsoWeek, Datelike};

fn main() {
    let now = Local::now();
    let isow = now.iso_week();

    println!("{}", isow.week_fancy());
}
```

## License

`isocal` is licensed under either of MIT or the Apache License, Version 2.0.

See the [LICENSE-MPL](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files in this repository for more information.

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.
