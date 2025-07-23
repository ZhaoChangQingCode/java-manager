use std::io::{self, Write};
use std::{
    ffi::CStr,
    fmt::{Debug, Display},
    os::raw::c_char,
    str::FromStr,
};

#[inline]
pub fn _stdin<F>(prmopt: impl Display) -> F
where
    F: FromStr,
    <F as FromStr>::Err: Debug,
{
    print!("{prmopt}");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read from stdin");
    buf.parse::<F>().expect("Failed to parse")
}
