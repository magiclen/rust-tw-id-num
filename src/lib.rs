/*!
# tw-id-num

Check or generate Taiwan ID numbers.

檢查或是產生中華民國**國民身分證統一編號**(即**身份證字號**，而非營業人統一編號)，支援**新式外來人口統一證號**。

## Usage

```rust
# #[cfg(feature = "generate")]
# {
let id = tw_id_num::generate_national(Some(tw_id_num::Sex::Male)); // e.g. "A123456789"

assert!(tw_id_num::check(&id));
assert!(tw_id_num::check_national(&id));
assert!(!tw_id_num::check_resident(&id));
# }
```

## no_std and the `generate` Feature

This crate can be compiled without std.

In few cases, you may want to generate IDs by yourself. The `generate*` functions are available if the `generate` feature is enabled. But the feature needs std.

```toml
[dependencies.tw-id-num]
version = "*"
features = ["generate"]
```
*/

#![cfg_attr(not(feature = "rand"), no_std)]

mod check;
mod location_values;

#[cfg(feature = "rand")]
mod generate;

pub use check::*;

#[cfg(feature = "rand")]
pub use generate::*;
