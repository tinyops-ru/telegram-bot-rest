#[macro_use]
extern crate log;
extern crate log4rs;

use std::env;

use crate::config::config::load_from_file;
use crate::logging::logging::get_logging_config;

const VERSION: &str = "0.1.0";

const TELEGRAM_API_BASE_URL: &str = "https://api.telegram.org/bot";

mod config;
mod config_tests;

mod logging;

const CONFIG_FILE: &str = "telegram-bot.conf";

#[tokio::main]
async fn main() {
    let working_directory = env::current_dir().expect("unable to get current working directory");

    let logging_config = get_logging_config(&working_directory.display().to_string());
    log4rs::init_config(logging_config).unwrap();

    match load_from_file(CONFIG_FILE) {
        Ok(config) => {
            info!("config has been loaded from file");

            let port = *&config.port;

            let routes = filters::routes(config);

            warp::serve(routes)
                .run(([127, 0, 0, 1], port))
                .await;
        }
        Err(_error) => println!("error: unable to load config from file")
    }
}

mod filters {
    use warp::Filter;

    use crate::config::config::Config;

    use super::handlers;

    pub fn routes(config: Config) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        return send_message(config)
               .or(get_version())
    }

    pub fn send_message(config: Config) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let rest_auth_token = String::from(config.rest_auth_token.clone());
        let authenticated = warp::header::exact("token", string_to_static_str(rest_auth_token));

        return  warp::path!("rest" / "send")
            .and(authenticated)
            .and(warp::post())
            .and(with_config(config.clone()))
            .and(warp::body::form())
            .and_then(handlers::send_message)
    }

    pub fn get_version() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        return warp::path!("version")
            .and(warp::get())
            .and_then(handlers::get_version)
    }

    fn with_config(config: Config) -> impl Filter<Extract = (Config,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || config.clone())
    }

    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }
}

mod handlers {
    use std::collections::HashMap;
    use std::convert::Infallible;
    use std::str::from_utf8;

    use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

    use crate::{TELEGRAM_API_BASE_URL, VERSION};
    use crate::config::config::Config;

    const MESSAGE_FORM_KEY: &str = "message";

    pub async fn get_version() -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::html(VERSION))
    }

    pub async fn send_message(config: Config, form: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
        let message_encoded = form.get(MESSAGE_FORM_KEY).unwrap();

        let message = base64::decode(message_encoded).unwrap();

        let message_decoded = from_utf8(&message).unwrap();

        debug!("decoded message '{}'", message_decoded);

        for chat_id in config.telegram_chat_ids {
            info!("send message to chat (id {})", chat_id);

            let request_url = get_send_message_url(&config.telegram_bot_token, chat_id, message_decoded);

            let resp: reqwest::Response = reqwest::get(&request_url).await.unwrap();
            let status: reqwest::StatusCode = resp.status();

            let response_text = resp.text().await.unwrap();

            debug!("response text '{}'", response_text);

            if status == reqwest::StatusCode::OK {
                info!("message has been sent");

            } else {
                println!("error, response code was {}", status)
            }
        }

        Ok(warp::reply::html(""))
    }

    fn get_send_message_url(auth_token: &str, chat_id: i32, message: &str) -> String {
        let url_encoded_message = utf8_percent_encode(message, NON_ALPHANUMERIC).to_string();
        return format!("{}{}/sendMessage?chat_id={}&disable_web_page_preview=true&text={}", TELEGRAM_API_BASE_URL, auth_token, chat_id, url_encoded_message)
    }
}
