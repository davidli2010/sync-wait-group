# sync-wait-group

[![Crate](https://img.shields.io/crates/v/sync-wait-group.svg)](https://crates.io/crates/sync-wait-group)
[![API](https://docs.rs/sync-wait-group/badge.svg)](https://docs.rs/sync-wait-group)

[![License: Apache](https://img.shields.io/badge/License-Apache%202.0-red.svg)](LICENSE-APACHE)
OR
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)

Wait group for synchronizing the beginning or end of some computation.

This crate is duplicated `WaitGroup` from `crossbeam_utils::sync::WaitGroup`, 
but use `parking_lot::{Mutex, Condvar}` instead of `std::sync::{Mutex, Condvar}`.

## Rust Version

This version of `sync-wait-group` requires Rust 1.56 or later.

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `sync-wait-group` by you, shall be licensed as Apache-2.0 and MIT, without any additional
terms or conditions.
