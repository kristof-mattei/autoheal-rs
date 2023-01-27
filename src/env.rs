fn parse_env_variable<T>(env_variable_name: &str) -> Result<Option<T>, anyhow::Error>
where
    T: std::str::FromStr + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    match std::env::var_os(env_variable_name)
        .map(|ct| ct.into_string().map(|s| str::parse::<T>(&s)))
    {
        Some(Ok(Ok(ct))) => Ok(Some(ct)),
        None => Ok(None),
        Some(Ok(Err(err))) => Err(anyhow::Error::msg(format!(
            "Could not parse {err:?} to requested type"
        )))?,
        Some(Err(err)) => Err(anyhow::Error::msg(format!(
            "Could not parse {err:?} to String"
        )))?,
    }
}

pub fn parse_optional_env_variable<T>(env_variable_name: &str) -> Result<Option<T>, anyhow::Error>
where
    T: std::str::FromStr + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
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
) -> Result<T, anyhow::Error>
where
    T: std::str::FromStr + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
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
