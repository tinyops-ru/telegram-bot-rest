#[cfg(test)]
mod config_tests {
    use crate::config::config::load_from_file;

    #[test]
    fn result_config_should_contain_rest_auth_token() {
        match load_from_file("tests/telegram-bot.conf") {
            Ok(config) => {
                assert_eq!(config.rest_auth_token, "53458092j45h9082j4gokjwlkfjg-23490gjq3049gj");
            }
            Err(_error) => panic!("property value expected")
        }
    }

    #[test]
    fn result_config_should_contain_telegram_bot_auth_token() {
        match load_from_file("tests/telegram-bot.conf") {
            Ok(config) => {
                assert_eq!(config.telegram_bot_token, "AAAA-BBN23894gq034gadkgas");
            }
            Err(_error) => panic!("property value expected")
        }
    }

    #[test]
    fn result_config_should_contain_telegram_chat_ids() {
        match load_from_file("tests/telegram-bot.conf") {
            Ok(config) => {
                let expected_chat_id1: i32 = 123456;
                assert_eq!(config.telegram_chat_ids.contains(&expected_chat_id1), true);
                let expected_chat_id2: i32 = 77712;
                assert_eq!(config.telegram_chat_ids.contains(&expected_chat_id2), true);
            }
            Err(_error) => panic!("property value expected")
        }
    }

    #[test]
    fn comments_should_be_ignored() {
        assert_eq!(load_from_file("tests/telegram-bot.conf").is_ok(), true)
    }

    #[test]
    #[should_panic]
    fn return_error_if_telegram_bot_auth_token_property_is_missing() {
        assert_eq!(load_from_file("tests/telegram-bot-missing-property1.conf").is_err(), true);
    }

    #[test]
    #[should_panic]
    fn return_error_if_rest_auth_token_property_is_missing() {
        assert_eq!(load_from_file("tests/telegram-bot-missing-property2.conf").is_err(), true);
    }

    #[test]
    #[should_panic]
    fn return_error_if_telegram_bot_chat_ids_is_missing() {
        assert_eq!(load_from_file("tests/telegram-bot-missing-chat-ids.conf").is_err(), true);
    }

    #[test]
    #[should_panic]
    fn return_error_if_telegram_bot_chat_ids_is_empty() {
        assert_eq!(load_from_file("tests/telegram-bot-empty-chat-ids.conf").is_err(), true);
    }
}
