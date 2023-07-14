#[macro_export]
macro_rules! wrap_and_report {
    ($level:expr, $error:expr, $message:expr) => {{
        let wrapped =
            color_eyre::Report::wrap_err(Into::<color_eyre::Report>::into($error), $message);

        tracing::event!($level, error = ?wrapped);

        wrapped
    }};
}
