#[macro_export]
macro_rules! jq {
    ($x:expr => $op:tt ) => {
        {
            use $crate::filter::Filter;
            $crate::jqop!($op).on_value($x)
        }
    };
}

#[macro_export]
macro_rules! jqop {
    ( "." ) => {
        $crate::filter::Identity
    }
}