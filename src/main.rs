#[macro_use]
extern crate log;
extern crate log4rs;

use std::env;

use crate::config::config::load_from_file;
use crate::filters::filters::routes;
use crate::logging::logging::get_logging_config;

const VERSION: &str = "0.1.0";

const TELEGRAM_API_BASE_URL: &str = "https://api.telegram.org/bot";

mod config;
mod config_tests;

mod filters;
mod handlers;

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

            let routes = routes(config);

            warp::serve(routes)
                .run(([127, 0, 0, 1], port))
                .await;
        }
        Err(_error) => println!("error: unable to load config from file")
    }
}
