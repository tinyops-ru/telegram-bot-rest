pub mod config {
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader};

    use regex::Regex;

    const DEFAULT_PORT: u16 = 31419;

    const PORT_PROPERTY: &str = "port";
    const REST_AUTH_TOKEN_PROPERTY: &str = "rest-auth-token";
    const TELEGRAM_BOT_AUTH_TOKEN_PROPERTY: &str = "telegram-bot.auth-token";
    const TELEGRAM_CHAT_IDS_PROPERTY: &str = "telegram-bot.chat-ids";

    #[derive(Clone)]
    pub struct Config {
        pub port: u16,
        pub rest_auth_token: String,
        pub telegram_bot_token: String,
        pub telegram_chat_ids: Vec<i32>
    }

    pub fn load_from_file(file_name: &str) -> Result<Config, io::Error> {
        info!("loading config from file '{}'", file_name);

        let input = File::open(file_name).expect("unable to load config from file");
        let buffered = BufReader::new(input);

        let mut port: u16 = DEFAULT_PORT;
        let mut rest_auth_token: String = String::from("");
        let mut telegram_bot_auth_token: String = String::from("");
        let mut telegram_chat_ids: Vec<i32> = Vec::new();

        let port_regex = get_property_regex(PORT_PROPERTY);
        let rest_auth_token_regex = get_property_regex(REST_AUTH_TOKEN_PROPERTY);
        let telegram_bot_auth_token_regex = get_property_regex(TELEGRAM_BOT_AUTH_TOKEN_PROPERTY);
        let telegram_chat_ids_regex = get_property_regex(TELEGRAM_CHAT_IDS_PROPERTY);

        for line in buffered.lines() {
            let row = line.unwrap();

            if port_regex.is_match(&row) {
                let groups_matches = port_regex.captures_iter(&row).next();
                let groups = groups_matches.unwrap();

                port = String::from(&groups[1]).parse::<u16>().unwrap();
                info!("port '{}'", port);
            }

            if rest_auth_token_regex.is_match(&row) {
                let groups_matches = rest_auth_token_regex.captures_iter(&row).next();
                let groups = groups_matches.unwrap();

                rest_auth_token = String::from(&groups[1]);
                info!("rest auth token '{}'", rest_auth_token);
            }

            if telegram_bot_auth_token_regex.is_match(&row) {
                let groups_matches = telegram_bot_auth_token_regex.captures_iter(&row).next();
                let groups = groups_matches.unwrap();

                telegram_bot_auth_token = String::from(&groups[1]);
                info!("telegram bot auth token '{}'", telegram_bot_auth_token);
            }

            if telegram_chat_ids_regex.is_match(&row) {
                let groups_matches = telegram_chat_ids_regex.captures_iter(&row).next();
                let groups = groups_matches.unwrap();

                let telegram_chat_ids_row = String::from(&groups[1]);

                let chat_ids: Vec<&str> = telegram_chat_ids_row.split("|").collect();

                for chat_id in chat_ids {
                    let chat_id_str: String = chat_id.to_string();
                    let value: i32 = chat_id_str.parse::<i32>().unwrap();
                    telegram_chat_ids.push(value);
                }

                info!("telegram chat ids '{}'", telegram_chat_ids_row);
            }
        }

        let config = Config {
            port,
            rest_auth_token: String::from(rest_auth_token),
            telegram_bot_token: String::from(telegram_bot_auth_token),
            telegram_chat_ids
        };

        if is_config_valid(&config) {
            Ok(config)

        } else { panic!("invalid configuration") }
    }

    fn get_property_regex(property_name: &str) -> Regex {
        let pattern = format!("^{}=(.*)", property_name);
        return Regex::new(&pattern).unwrap();
    }

    fn is_config_valid(config: &Config) -> bool {
        let mut result = false;

        if config.rest_auth_token != "" && config.telegram_bot_token != "" &&
           !config.telegram_chat_ids.is_empty() {
            result = true

        } else { error!("one or more configuration properties are missing") }

        return result;
    }
}
