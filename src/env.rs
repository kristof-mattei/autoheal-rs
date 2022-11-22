pub fn parse_env_variable<T>(env_variable_name: &str) -> Result<Option<T>, anyhow::Error>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    match std::env::var_os(env_variable_name)
        .map(|ct| ct.into_string().map(|s| str::parse::<T>(&s)))
    {
        Some(Ok(Ok(ct))) => Ok(Some(ct)),
        None => {
            println!("{env_variable_name} not set");
            Ok(None)
        },
        Some(Ok(Err(err))) => Err(anyhow::Error::msg(format!(
            "Could not parse {:?} to requested type",
            err
        )))?,
        Some(Err(err)) => Err(anyhow::Error::msg(format!(
            "Could not parse {:?} to String",
            err
        )))?,
    }
}

pub fn parse_env_variable_with_default<T>(
    env_variable_name: &str,
    default: T,
) -> Result<T, anyhow::Error>
where
    T: std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    match parse_env_variable(env_variable_name) {
        Ok(Some(ct)) => Ok(ct),
        Ok(None) => {
            println!("{env_variable_name} not set, defaulting to {default}");
            Ok(default)
        },
        Err(e) => Err(e),
    }
}
