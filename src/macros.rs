#[macro_export]
macro_rules! NSDictionary {
    // Handle core syntax, with and without trailing comma.
    { $( $key:expr => $value:expr ),* } => {
        NSDictionary!(1 @ $([$key => $value])* | &[] | &[])
    };
    { $( $key:expr => $value:expr,)* } => {
        NSDictionary!(1 @ $([$key => $value])* | &[] | &[])
    };
    // Loop to split "key => value" pairs into lists of keys and values.
    { 1 @ [$key:expr => $value:expr] $([$rest_keys:expr => $rest_values:expr])*
        | &[$($keys:expr,)*] | &[$($values:expr,)*]
    } => {
        NSDictionary!(1 @ $([$rest_keys => $rest_values])* | &[$($keys,)* $key,] | &[$($values,)*  $value,])
    };
    // Once the list of pairs is empty, we're ready for lowering.
    { 1 @ | &[$($keys:expr,)*] | &[$($values:expr,)*] } => {
        {
            // We use slices to avoid running into issues with fixed-size arrays.
            // The slices still have sizes known at compilation time,
            // so I expect this code to be optimized out.
            let keys = &[$($keys,)*];
            let values = &[$($values,)*];
            let count = std::convert::TryInto::try_into(keys.len()).expect("too big dictionary");
            cocoa::foundation::NSDictionary::dictionaryWithObjects_forKeys_count_(
                nil,
                values.as_ptr(),
                keys.as_ptr(),
                count,
            )
        }
    };
}

#[macro_export]
macro_rules! NSString {
    { $str:expr } => {
        {
            cocoa::foundation::NSString::alloc(nil).init_str(AsRef::<str>::as_ref(&$str))
        }
    };
}
