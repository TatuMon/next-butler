use next_butler::user_config::UserConfig;

#[test]
fn test_user_config_deserialize() {
    let deserialized_user_config = UserConfig::get();

    if let Err(err) = deserialized_user_config {
        panic!("{}", err);
    }
}

