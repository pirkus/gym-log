#[macro_export]
macro_rules! log_panic {
    ($($arg:tt)*) => {{
        log::error!($($arg)*);
        core::panic!($($arg)*)
    }}
}