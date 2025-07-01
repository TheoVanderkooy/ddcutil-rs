// TODO make sure these don't get exported outside the crate?

/// Convenience macro to generate getters for [c_int; len] fields that represent strings
///
/// Usage:
///
/// ```
/// # use libddcutil2::str_field_getter;
/// struct Inner{ str_field: [::std::os::raw::c_char; 10] }
/// struct Wrapper(*mut Inner);
///
/// impl Wrapper {
///     // generates `get_example(&self) -> &str` for Inner::str_field
///     str_field_getter!(get_example, str_field);
/// }
/// ```
#[macro_export]
macro_rules! str_field_getter {
    ($fn_name:ident, $field:ident) => {
        pub fn $fn_name<'a>(&'a self) -> &'a str {
            // interpret the bytes as &[u8]
            let buf =
                unsafe { &*(&self.0.$field as *const [::std::os::raw::c_char] as *const [u8]) };

            // find the length: position of first null terminator OR full length if none
            let end = buf.iter().position(|&c| c == 0x0).unwrap_or(buf.len());

            // build &str from the slice
            std::str::from_utf8(&buf[0..end])
                .unwrap_or_else(|_| panic!("could not decode {0}: {1:?}", stringify!($field), buf))
        }
    };
}
