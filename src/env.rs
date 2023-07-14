fn parse_env_variable<T>(env_variable_name: &str) -> Result<Option<T>, color_eyre::Report>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::error::Error,
    <T as std::str::FromStr>::Err: std::marker::Send,
    <T as std::str::FromStr>::Err: std::marker::Sync,
    <T as std::str::FromStr>::Err: 'static,
{
    match std::env::var_os(env_variable_name)
        .map(|ct| ct.into_string().map(|s| str::parse::<T>(&s)))
    {
        Some(Ok(Ok(ct))) => Ok(Some(ct)),
        None => Ok(None),
        Some(Ok(Err(err))) => Err(color_eyre::Report::wrap_err(
            err.into(),
            format!(
                "Env variable {:?} could not be cast to requested type",
                env_variable_name
            ),
        )),
        Some(Err(err)) => Err(color_eyre::Report::msg(format!(
            "Env variable {:?} could not be cast to String. Orignal value is {:?}",
            env_variable_name, err
        ))),
    }
}

pub fn parse_optional_env_variable<T>(
    env_variable_name: &str,
) -> Result<Option<T>, color_eyre::Report>
where
    T: std::fmt::Debug,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::error::Error,
    <T as std::str::FromStr>::Err: std::marker::Send,
    <T as std::str::FromStr>::Err: std::marker::Sync,
    <T as std::str::FromStr>::Err: 'static,
{
    match parse_env_variable(env_variable_name) {
        Ok(Some(ct)) => {
            tracing::info!("{} set to {:?}", env_variable_name, ct);
            Ok(Some(ct))
        },
        Ok(None) => {
            tracing::info!("{} not set", env_variable_name);
            Ok(None)
        },
        Err(e) => Err(e),
    }
}

pub fn parse_env_variable_with_default<T>(
    env_variable_name: &str,
    default: T,
) -> Result<T, color_eyre::Report>
where
    T: std::fmt::Debug,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::error::Error,
    <T as std::str::FromStr>::Err: std::marker::Send,
    <T as std::str::FromStr>::Err: std::marker::Sync,
    <T as std::str::FromStr>::Err: 'static,
{
    match parse_env_variable(env_variable_name) {
        Ok(Some(ct)) => {
            tracing::info!("{} set to {:?}", env_variable_name, ct);
            Ok(ct)
        },

        Ok(None) => {
            tracing::info!("{} not set, defaulting to {:?}", env_variable_name, default);
            Ok(default)
        },
        Err(e) => Err(e),
    }
}
