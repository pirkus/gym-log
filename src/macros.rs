#[macro_export]
macro_rules! log_panic {
    ($($arg:tt)*) => {{
        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
        error!(res);
        panic!(res);
    }}
}

// error!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
// error!(target: "my_target", "a {} event", "log")
// (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Error, $($arg)+));

// error!("a {} event", "log")
// ($($arg:tt)+) => ($crate::log!($crate::Level::Error, $($arg)+))