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