#![cfg_attr(not(debug_assertions), deny(warnings))]

use std::ffi::OsString;
use hyperlocal::{UnixClientExt};

mod api_config;

use crate::api_config::{ApiConfig, build_api_config};

fn parse_env_variable<T>(env_variable_name: &str, default: T) -> Result<T, anyhow::Error>
where
    T: std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    match std::env::var_os(env_variable_name)
        .map(|ct| ct.into_string().map(|s| str::parse::<T>(&s)))
    {
        Some(Ok(Ok(ct))) => Ok(ct),
        None => {
            println!("{env_variable_name} not set, defaulting to {default}");
            Ok(default)
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

fn main() {
    // set up logger
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new(
    //         std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
    //     ))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    // initialize the runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    // start service
    let result: Result<(), anyhow::Error> = rt.block_on(actual_main());

    // if let Err(err) = result {
    //     error!("{:?}", err);
    // }
}

async fn actual_main() -> Result<(), anyhow::Error> {
    let docker_sock = std::env::var_os("DOCKER_SOCK")
        .map_or_else(
            || Ok(String::from("/var/run/docker.sock")),
            OsString::into_string,
        )
        .map_err(|err| anyhow::Error::msg(format!("Could not convert {:?} to String", err)))?;

    let webhook_url = std::env::var("WEBHOOK_URL").map(String::from).ok();

    let config = AppConfig {
        api_config: build_api_config(docker_sock),
        curl_timeout: parse_env_variable("CURL_TIMEOUT", 30)?,
        webhook_url,
        autoheal_container_label: parse_env_variable(
            "AUTOHEAL_CONTAINER_LABEL",
            String::from("autoheal"),
        )?,
        autoheal_start_period: parse_env_variable("AUTOHEAL_START_PERIOD", 0)?,
        autoheal_interval: parse_env_variable("AUTOHEAL_INTERVAL", 5)?,
        autoheal_default_stop_timeout: parse_env_variable("AUTOHEAL_DEFAULT_STOP_TIMEOUT", 10)?,
    };

    let container_info = get_container_info(&config).await;

    Ok(())
}

struct AppConfig {
    api_config: ApiConfig,
    curl_timeout: u32,
    webhook_url: Option<String>,
    autoheal_container_label: String,
    autoheal_start_period: u32,
    autoheal_interval: u32,
    autoheal_default_stop_timeout: u32,
}

struct ContainerInfo {}

async fn docker_curl(config: &AppConfig, uri: hyper::Uri) -> Result<String, anyhow::Error> {
    let client = hyper::Client::unix();

    let body = client.get(uri).await?.into_body();
    let bytes = hyper::body::to_bytes(body).await?.to_vec();

    let deserialized: serde_json::Value = serde_json::from_slice(&bytes)?;

    println!("hello: {}", deserialized);
    // .and_then(|res| {
    //     res.into_body().concat2()
    // })
    // .and_then(|body| {
    //     // try to parse as json, perhaps with serde_json
    //     serde_json::from_slice(&body)
    //         .map_err(|e| handle_json_error(e))
    // })

    //   curl --max-time "${CURL_TIMEOUT}" --no-buffer -s \
    //   ${CA} ${CLIENT_KEY} ${CLIENT_CERT} \
    //   ${UNIX_SOCK} \
    //   "$@"
    Ok(String::from("done"))
}

async fn get_container_info(config: &AppConfig) -> ContainerInfo {
    // let label_filter = if "all" == config.autoheal_container_label {
    //     String::from("")
    // } else {
    //     format!(",\"label\":[\"{}=true\"]", config.autoheal_container_label)
    // };

    let label_filter = "";

    let encoded = percent_encoding::percent_encode(
        format!("{{\"health\":[\"none\"]{}}}", label_filter).as_bytes(),
        percent_encoding::NON_ALPHANUMERIC,
    )
    .to_string();

    let path_and_query = format!("containers/json?filters={}", encoded);

    let uri: hyper::Uri = format!("{}{}", config.api_config.endpoint, path_and_query)
        .parse()
        .unwrap();

    println!("{:?}", &uri);

    // get URL over ApiConfig
    let r = docker_curl(config, uri).await;
    ContainerInfo {}
}
fn restart_container(constainer_id: String, timeout: u32) {
    //   local container_id="$1"
    //   local timeout="$2"
    //
    //   docker_curl -f -X POST "${HTTP_ENDPOINT}/containers/${container_id}/restart?t=${timeout}"
}
//
fn notify_webhook() {
    //   local text="$@"
    //
    //   if [ -n "$WEBHOOK_URL" ]
    //   then
    //     # execute webhook requests as background process to prevent healer from blocking
    //     curl -X POST -H "Content-type: application/json" -d "$(generate_webhook_payload $text)"  $WEBHOOK_URL
    //   fi
}

// # https://towardsdatascience.com/proper-ways-to-pass-environment-variables-in-json-for-curl-post-f797d2698bf3
fn generate_webhook_payload() {
    //   local text="$@"
    //   cat <<EOF
    // {
    //   "text":"$text"
    // }
    // EOF
}
//
// SIGTERM-handler
fn term_handler() {
    //   exit 143  # 128 + 15 -- SIGTERM
}
//
// # shellcheck disable=2039
// trap 'kill $$; term_handler' SIGTERM
//
// if [ "$1" = "autoheal" ] && [ -e "$DOCKER_SOCK" ];then
//   # Delayed startup
//   if [ "$AUTOHEAL_START_PERIOD" -gt 0 ]
//   then
//   echo "Monitoring containers for unhealthy status in $AUTOHEAL_START_PERIOD second(s)"
//     sleep "$AUTOHEAL_START_PERIOD"
//   fi
//
//   while true
//   do
//     STOP_TIMEOUT=".Labels[\"autoheal.stop.timeout\"] // $AUTOHEAL_DEFAULT_STOP_TIMEOUT"
//     get_container_info | \
//       jq -r "foreach .[] as \$CONTAINER([];[]; \$CONTAINER | .Id, .Names[0], .State, ${STOP_TIMEOUT})" | \
//       while read -r CONTAINER_ID && read -r CONTAINER_NAME && read -r CONTAINER_STATE && read -r TIMEOUT
//     do
//       # shellcheck disable=2039
//       CONTAINER_SHORT_ID=${CONTAINER_ID:0:12}
//       DATE=$(date +%d-%m-%Y" "%H:%M:%S)
//
//       if [ "$CONTAINER_NAME" = "null" ]
//       then
//         echo "$DATE Container name of (${CONTAINER_SHORT_ID}) is null, which implies container does not exist - don't restart" >&2
//       elif [ "$CONTAINER_STATE" = "restarting" ]
//       then
//         echo "$DATE Container $CONTAINER_NAME (${CONTAINER_SHORT_ID}) found to be restarting - don't restart"
//       else
//         echo "$DATE Container $CONTAINER_NAME (${CONTAINER_SHORT_ID}) found to be unhealthy - Restarting container now with ${TIMEOUT}s timeout"
//         if ! restart_container "$CONTAINER_ID" "$TIMEOUT"
//         then
//           echo "$DATE Restarting container $CONTAINER_SHORT_ID failed" >&2
//           notify_webhook "Container ${CONTAINER_NAME:1} (${CONTAINER_SHORT_ID}) found to be unhealthy. Failed to restart the container!" &
//         else
//           notify_webhook "Container ${CONTAINER_NAME:1} (${CONTAINER_SHORT_ID}) found to be unhealthy. Successfully restarted the container!" &
//         fi
//       fi
//     done
//     sleep "$AUTOHEAL_INTERVAL"
//   done
//
// else
//   exec "$@"
// fi
//
