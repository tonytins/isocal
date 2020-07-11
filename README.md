# IsoCal

![Rust](https://github.com/tonytins/isocal/workflows/Rust/badge.svg) [![Build Status](https://travis-ci.org/tonytins/isocal.svg?branch=master)](https://travis-ci.org/tonytins/isocal)

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

## Authors

- **Anthony Foxclaw** - _Initial work_ - [tonytins](https://github.com/tonytins)

See also the list of [contributors](https://github.com/tonytins/isow/contributors) who participated in this project.

## License

`isocal` is licensed under either of MIT or the Mozilla Public License, Version 2.0.

See the [LICENSE-MPL](LICENSE-MPL) and [LICENSE-MIT](LICENSE-MIT) files in this repository for more information.
