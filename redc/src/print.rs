
#[macro_export]
macro_rules! ok {
    ($message:expr) => {
        ok!("OK", $message)
    };
    ($level:expr, $message:expr) => {
        println!(
            "{}{:>12}{} {}",
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
            $level,
            anstyle::Reset.render(),
            $message
        )
    };
    ($level:expr, $message:expr, $($arg:tt)*) => {
        ok!($level, format!($message, $($arg)*))
    };
}

#[macro_export]
macro_rules! err {
    ($message:expr) => {
        eprintln!(
            "{}error:{} {}",
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
            anstyle::Reset.render(),
            $message
        )
    };
    ($message:expr, $($arg:tt)*) => {
        err!(format!($message, $($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        eprintln!(
            "{}warning:{} {}",
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
            anstyle::Reset.render(),
            $message
        )
    };
    ($message:expr, $($arg:tt)*) => {
        warn!(format!($message, $($arg)*))
    };
}
