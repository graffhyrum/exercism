#[macro_export]
macro_rules! hashmap {
    () => {crate::HashMap::new()};
    ( $($x:expr=>$y:expr),+$(,)?) => {
        {
            let mut hm = crate::HashMap::new();
        $(
            hm.insert($x, $y);
        )*
        hm
        }
    };
}
