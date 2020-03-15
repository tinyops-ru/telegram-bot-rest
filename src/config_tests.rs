#[cfg(test)]
mod config_tests {
    use std::fs::File;
    use std::path::Path;

    use crate::config::config::load_from_file;

    #[test]
    fn result_config_should_contain_rest_auth_token() {
        let file = File::open("tests/telegram-bot.conf");

        match load_from_file("tests/telegram-bot.conf") {
            Ok(config) => {
                assert_eq!(config.rest_auth_token, "53458092j45h9082j4gokjwlkfjg-23490gjq3049gj");
            }
            Err(_error) => {
                panic!("property value expected")
            }
        }
    }

    #[test]
    fn result_config_should_contain_telegram_bot_auth_token() {
        let file = File::open("tests/telegram-bot.conf");

        match load_from_file("tests/telegram-bot.conf") {
            Ok(config) => {
                assert_eq!(config.telegram_bot_token, "AAAA-BBN23894gq034gadkgas");
            }
            Err(_error) => {
                panic!("property value expected")
            }
        }
    }

    #[test]
    fn comments_should_be_ignored() {
        assert_eq!(load_from_file("tests/telegram-bot.conf").is_ok(), true)
    }

    #[test]
    #[should_panic]
    fn return_error_if_telegram_bot_auth_token_property_is_missing() {
        load_from_file("tests/telegram-bot-missing-property1.conf");
    }

    #[test]
    #[should_panic]
    fn return_error_if_rest_auth_token_property_is_missing() {
        load_from_file("tests/telegram-bot-missing-property2.conf");
    }
}
