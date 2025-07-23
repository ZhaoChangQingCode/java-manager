#[macro_export]
macro_rules! warnx {
    ($fmt:expr) => {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), $fmt)
    };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! warnx_err {
    ($fmt:expr) => {

    };
}

#[macro_export]
macro_rules! stdin {
    ($ty:ty, $fmt:expr, $($arg:tt)*) => {{
        io2::_stdin::<$ty>(format!($fmt, $($arg)*))
    }};
}
