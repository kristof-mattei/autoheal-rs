#[macro_export]
macro_rules! wrap_and_report {
    ($level:expr_2021, $error:expr_2021, $message:expr_2021) => {{
        let wrapped =
            color_eyre::Report::wrap_err(Into::<color_eyre::Report>::into($error), $message);

        tracing::event!($level, error = ?wrapped);

        wrapped
    }};
}
