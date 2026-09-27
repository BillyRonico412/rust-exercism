#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

/// This module contains doctests, which allows writing tests where a code
/// snippet is supposed to fail to compile. These tests also have "ignore"
/// attributes, make sure to remove them when solving this exercise locally.
pub mod compile_fail_tests;
