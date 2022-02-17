macro_rules! kv_array {
    ([$($list:expr),*] + ) => ([$($list),*]);
    ([$($list:expr),*] + $key:expr => $value:expr, $($rest:tt)*) => (
        [$($list),* ($key, $value)]
    );
    ($($rest:tt)*) => (kv_array!([] + $($rest)*));
}

macro_rules! dict {
    ($($key:expr => $value:expr),*) => (
        stringify!($($key => $value),*)
        //std::collections::HashMap::from(kv_array!($($key => $value),*))
    );
}