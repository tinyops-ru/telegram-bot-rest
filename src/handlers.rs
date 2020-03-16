pub mod handlers {
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