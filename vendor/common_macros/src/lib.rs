//! This library contains common macros.
//!
//! As this crate exports many different macros it is recommended
//! not to use `#[macro_use] extern crate common_macros;` but instead
//! to use the newer way of "just" importing the macros you need
//! (e.g. `use common_macros::{hash_map, hash_set};`).
//!


/// Counts the number of `;` separated expressions passed in to the
/// macro at compiler time.
///
/// This does count expressions containing other expressions just as
/// one expression.
///
/// The macro can be used to e.g. get the right value for a `.with_capacity()`
/// constructor at compiler time, i.e. it's normally used only by other macros.
///
///
/// # Examples
///
/// ```
/// use common_macros::const_expr_count;
///
/// macro_rules! my_vec {
///     ($($item:expr),*) => ({
///         let mut vec = Vec::with_capacity(const_expr_count! {$($item);*});
///         $(
///             //let's forget to insert the values
///             if false {
///                 vec.push($item);
///             }
///         )*
///         vec
///     });
/// }
///
/// fn main() {
///     let vec = my_vec![1u8,2,3,4];
///     assert!(vec.capacity() >= 4);
/// }
/// ```
#[macro_export]
macro_rules! const_expr_count {
    () => (0);
    ($e:expr) => (1);
    ($e:expr; $($other_e:expr);*) => ({
        1 $(+ $crate::const_expr_count!($other_e) )*
    });

    ($e:expr; $($other_e:expr);* ; ) => (
        $crate::const_expr_count! { $e; $($other_e);* }
    );
}

/// Macro to crate a `HashMap` with a number of key-value pairs in it.
///
/// There is an alternate syntax, which allows insertion into an existing
/// map (and still uses `const_expr_count!` to call `map.reserve()` with).
///
/// # Examples
///
/// ```
/// use common_macros::hash_map;
///
/// let is_fun_map = hash_map!{
///     "joke" => true,
///     "cat" => true,
/// };
/// ```
///
/// ```
/// use std::collections::HashMap;
/// use common_macros::hash_map;
///
/// fn setup_map() -> HashMap<u8,u8> {
///     HashMap::new()
/// }
///
/// let mut map = hash_map!(with setup_map(); insert {
///     12 => 13,
///     13 => 56
/// });
///
/// hash_map!(with &mut map; insert {
///     44 => 45,
///     48 => 112,
///     1 => 4
/// });
/// ```
#[macro_export]
macro_rules! hash_map {
    (with $map:expr; insert { $($key:expr => $val:expr),* , }) => (
        $crate::hash_map!(with $map; insert { $($key => $val),* })
    );
    (with $map:expr; insert { $($key:expr => $val:expr),* }) => ({
        let count = $crate::const_expr_count!($($key);*);
        #[allow(unused_mut)]
        let mut map = $map;
        map.reserve(count);
        $(
            map.insert($key, $val);
        )*
        map
    });
    ($($key:expr => $val:expr),* ,) => (
        $crate::hash_map!($($key => $val),*)
    );
    ($($key:expr => $val:expr),*) => ({
        let start_capacity = $crate::const_expr_count!($($key);*);
        #[allow(unused_mut)]
        let mut map = ::std::collections::HashMap::with_capacity(start_capacity);
        $( map.insert($key, $val); )*
        map
    });
}

/// Macro to create a `HashSet` with a number of items in it.
///
/// Like for `HashMap` an alternate syntax for insertion exists,
/// it also call `.reserve()` with the number of passed in items.
///
/// # Examples
///
/// ```
/// use common_macros::hash_set;
///
/// let is_fun_set = hash_set!{ "joke", "cat" };
/// ```
///
/// ```
/// use std::collections::HashSet;
/// use common_macros::hash_set;
///
/// let mut set = HashSet::new();
/// hash_set!(with &mut set; insert {
///     "hy",
///     "ho",
///     "starts",
///     "and",
///     "so"
/// });
/// ```
///
#[macro_export]
macro_rules! hash_set {
    (with $set:expr; insert { $($item:expr),* , }) => (
        $crate::hash_set!(with $set; insert { $($item),* })
    );
    (with $set:expr; insert { $($item:expr),* }) => ({
        let count = $crate::const_expr_count!($($item);*);
        #[allow(unused_mut)]
        let mut set = $set;
        set.reserve(count);
        $(
            set.insert($item);
        )*
        set
    });
    ($($item:expr),* ,) => (
        $crate::hash_set!($($item),*)
    );
    ($($item:expr),*) => ({
        let start_capacity = $crate::const_expr_count!($($item);*);
        #[allow(unused_mut)]
        let mut set = ::std::collections::HashSet::with_capacity(start_capacity);
        $( set.insert($item); )*
        set
    });
}

/// Macro to crate a `BTreeMap` with a number of key-value pairs in it.
///
/// # Examples
///
/// ```
/// use common_macros::b_tree_map;
///
/// let is_fun_map = b_tree_map!{
///     "joke" => true,
///     "cat" => true,
/// };
/// ```
///
/// Like `HashMap` and alternative insertion syntax exists:
///
/// ```
/// use common_macros::b_tree_map;
///
/// let is_fun_map = b_tree_map!{
///     "joke" => true
/// };
///
/// let mut less_fun_map = b_tree_map!{
///     with is_fun_map; insert {
///         "explosion" => false,
///         "psychos" => false
///     }
/// };
///
/// b_tree_map!{
///     with &mut less_fun_map; insert {
///         "cat" => true,
///         "mosquito" => false
///     }
/// };
/// ```
#[macro_export]
macro_rules! b_tree_map {
    (with $map:expr; insert { $($key:expr => $val:expr),* , }) => (
        $crate::b_tree_map!(with $map; insert { $($key => $val),* })
    );
    (with $map:expr; insert { $($key:expr => $val:expr),* }) => ({
        #[allow(unused_mut)]
        let mut map = $map;
        $(
            map.insert($key, $val);
        )*
        map
    });
    ($($key:expr => $val:expr),* ,) => (
        $crate::b_tree_map!($($key => $val),*)
    );
    ($($key:expr => $val:expr),*) => ({
        #[allow(unused_mut)]
        let mut map = ::std::collections::BTreeMap::new();
        $( map.insert($key, $val); )*
        map
    });
}

/// Macro to create a `BTreeSet` with a number of items in it.
///
/// # Examples
///
/// ```
/// use common_macros::b_tree_set;
///
/// let is_fun_set = b_tree_set!{ "joke", "cat" };
/// ```
#[macro_export]
macro_rules! b_tree_set {
    (with $set:expr; insert { $($item:expr),* , }) => (
        $crate::b_tree_set!(with $set; insert { $($item),* })
    );
    (with $set:expr; insert { $($item:expr),* }) => ({
        #[allow(unused_mut)]
        let mut set = $set;
        $(
            set.insert($item);
        )*
        set
    });
    ($($item:expr),* ,) => (
        $crate::b_tree_set!($($item),*)
    );
    ($($item:expr),*) => ({
        #[allow(unused_mut)]
        let mut set = ::std::collections::BTreeSet::new();
        $( set.insert($item); )*
        set
    });
}

/// Concats a number of string literals inserting a "\n" between each of them.
#[macro_export]
macro_rules! lines {
    ($first:expr $(, $tail:expr)*) => (
        concat!($first $(, "\n", $tail)*)
    );
}

#[cfg(test)]
mod tests {

    mod const_expr_count {

        #[test]
        fn zero_expression() {
            assert_eq!(const_expr_count!{}, 0u8);
        }

        #[test]
        fn one_expression() {
            assert_eq!(const_expr_count!{1}, 1u8);
        }

        #[test]
        fn one_expression_with_semicolon() {
            assert_eq!(const_expr_count!{1;}, 1u8);
        }

        #[test]
        fn multiple_expressions() {
            assert_eq!(const_expr_count!{1; 1+2; (3+4, 5)}, 3u8);
        }

        #[test]
        fn multiple_expressions_with_trailing_semicolon() {
            assert_eq!(const_expr_count!{1; 1+2; (3+4, 5);}, 3u8);
        }
    }

    mod hash_map {
        use std::collections::HashMap;

        #[test]
        fn create_empty() {
            let map: HashMap<u8, u8> = hash_map!();
            assert_eq!(map.len(), 0);
        }

        #[test]
        fn create_non_empty() {
            let map = hash_map!{
                1u8 => 2u32
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.len(), 1);

            let map = hash_map!{
                1u8 => 2u32,
                4u8 => 12u32
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.get(&4), Some(&12));
            assert_eq!(map.len(), 2);
        }

        #[test]
        fn create_non_empty_with_tailing_comma() {
            let map = hash_map!{
                1u8 => 2u32,
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.len(), 1);
        }

        #[test]
        fn can_insert_instead_of_create() {
            let mut map = HashMap::new();

            hash_map!{
                with &mut map; insert {
                    1u8 => 2u8,
                    2 => 12,
                }
            };

            let map = hash_map!{
                with map; insert {
                    12 => 32
                }
            };

            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.get(&2), Some(&12));
            assert_eq!(map.get(&12), Some(&32));
            assert_eq!(map.len(), 3);
        }
    }

    mod hash_set {
        use std::collections::HashSet;

        #[test]
        fn create_empty() {
            let set: HashSet<u8> = hash_set!();
            assert_eq!(set.len(), 0);
        }

        #[test]
        fn create_non_empty() {
            let set = hash_set!{ 1u8 };
            assert!(set.contains(&1));
            assert_eq!(set.len(), 1);

            let set = hash_set!{ 1u8, 4u8 };
            assert!(set.contains(&1));
            assert!(set.contains(&4));
            assert_eq!(set.len(), 2);
        }

        #[test]
        fn create_non_empty_with_tailing_comma() {
            let set = hash_set!{ 1u8, };
            assert!(set.contains(&1));
            assert_eq!(set.len(), 1);
        }

        #[test]
        fn can_insert_instead_of_create() {
            let mut set = HashSet::new();

            hash_set!{with &mut set; insert { 1u8, 2 }};

            let set = hash_set!{with set; insert { 12 }};
            assert!(set.contains(&1));
            assert!(set.contains(&2));
            assert!(set.contains(&12));
            assert_eq!(set.len(), 3);
        }
    }

    mod b_tree_map {
        use std::collections::BTreeMap;

        #[test]
        fn create_empty() {
            let map: BTreeMap<u8, u8> = b_tree_map!();
            assert_eq!(map.len(), 0);
        }

        #[test]
        fn create_non_empty() {
            let map = b_tree_map!{
                1u8 => 2u32
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.len(), 1);

            let map = b_tree_map!{
                1u8 => 2u32,
                4u8 => 12u32
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.get(&4), Some(&12));
            assert_eq!(map.len(), 2);
        }

        #[test]
        fn create_non_empty_with_tailing_comma() {
            let map = b_tree_map!{
                1u8 => 2u32,
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.len(), 1);
        }

        #[test]
        fn can_insert_instead_of_create() {
            let mut map = BTreeMap::new();

            b_tree_map!{
                with &mut map; insert {
                    1u8 => 2u8,
                    2 => 12,
                }
            };

            let map = b_tree_map!{
                with map; insert {
                    12 => 32
                }
            };

            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.get(&2), Some(&12));
            assert_eq!(map.get(&12), Some(&32));
            assert_eq!(map.len(), 3);
        }
    }

    mod b_tree_set {
        use std::collections::BTreeSet;

        #[test]
        fn create_empty() {
            let set: BTreeSet<u8> = b_tree_set!();
            assert_eq!(set.len(), 0);
        }

        #[test]
        fn create_non_empty() {
            let set = b_tree_set!{ 1u8 };
            assert!(set.contains(&1));
            assert_eq!(set.len(), 1);

            let set = b_tree_set!{ 1u8, 4u8 };
            assert!(set.contains(&1));
            assert!(set.contains(&4));
            assert_eq!(set.len(), 2);
        }

        #[test]
        fn create_non_empty_with_tailing_comma() {
            let set = b_tree_set!{ 1u8, };
            assert!(set.contains(&1));
            assert_eq!(set.len(), 1);
        }

        #[test]
        fn can_insert_instead_of_create() {
            let mut set = BTreeSet::new();

            b_tree_set!{with &mut set; insert { 1u8, 2 }};

            let set = b_tree_set!{with set; insert { 12 }};
            assert!(set.contains(&1));
            assert!(set.contains(&2));
            assert!(set.contains(&12));
            assert_eq!(set.len(), 3);
        }
    }

    mod lines {

        #[test]
        fn handles_single_line() {
            let res = lines!{"hy"};
            assert_eq!(res, "hy");
        }

        #[test]
        fn inserts_newline_between_lines() {
            let res = lines!{ "hy", "there" };
            assert_eq!(res, "hy\nthere");
        }

        #[test]
        fn can_handle_raw_string_literals() {
            let res = lines!{ "hy", r#"th"ere"# };
            assert_eq!(res, "hy\nth\"ere");
        }
    }
}