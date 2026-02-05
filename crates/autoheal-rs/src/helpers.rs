#[macro_export]
macro_rules! wrap_and_report {
    ($level:expr, $error:expr, $message:expr) => {{
        let wrapped =
            eyre::Report::wrap_err(Into::<eyre::Report>::into($error), $message);

        tracing::event!($level, error = ?wrapped);

        wrapped
    }};
}
