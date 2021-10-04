# common_macros [![Crates.io](https://img.shields.io/crates/v/common_macros.svg)](https://crates.io/crates/common_macros) [![common_macros](https://docs.rs/common_macros/badge.svg)](https://docs.rs/common_macros) [![maintenance](https://img.shields.io/badge/maintenance-passively--maintained-blue.svg)](https://img.shields.io/badge/maintenance-passively--maintained-blue.svg) [![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**provides common macros like `hash_map!`

---

Rust crate providing some common macros.

Currently following macros are exported:

- `hash_map!`
- `hash_set!`
- `b_tree_map!`
- `b_tree_set!`
- `const_expr_count!`

## Example

```rust
use std::collections::HashMap;
use common_macros::hash_map;

fn main() {
    let map_a = hash_map! {
        "foo" => vec![0,1,2],
        "bar" => vec![3,4,5]
    };

    // expands to roughly
    let map_b = {
        let mut map = HashMap::with_capacity(2);
        map.insert("foo", vec![0,1,2]);
        map.insert("bar", vec![3,4,5]);
        map
    };

    assert_eq!(map_a, map_b);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.