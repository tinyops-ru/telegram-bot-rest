pub mod config {
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader};

    use regex::Regex;

    const REST_AUTH_TOKEN_PROPERTY: &str = "rest-auth-token";
    const TELEGRAM_BOT_AUTH_TOKEN_PROPERTY: &str = "telegram-bot.auth-token";

    pub struct Config {
        pub rest_auth_token: String,
        pub telegram_bot_token: String
    }

    pub fn load_from_file(file_name: &str) -> Result<Config, io::Error> {
        info!("loading config from file '{}'", file_name);

        let input = File::open(file_name).expect("unable to load config from file");
        let buffered = BufReader::new(input);

        let mut rest_auth_token: String = String::from("");
        let mut telegram_bot_auth_token: String = String::from("");

        let rest_auth_token_regex = get_property_regex(REST_AUTH_TOKEN_PROPERTY);
        let telegram_bot_auth_token_regex = get_property_regex(TELEGRAM_BOT_AUTH_TOKEN_PROPERTY);

        for line in buffered.lines() {
            let row = line.unwrap();

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
        }

        if rest_auth_token != "" && telegram_bot_auth_token != "" {
            Ok(
                Config {
                    rest_auth_token: String::from(rest_auth_token),
                    telegram_bot_token: String::from(telegram_bot_auth_token)
                }
            )

        } else {
            panic!("one or more configuration properties are missing")
        }
    }

    fn get_property_regex(property_name: &str) -> Regex {
        let pattern = format!("^{}=(.*)", property_name);
        return Regex::new(&pattern).unwrap();
    }
}
