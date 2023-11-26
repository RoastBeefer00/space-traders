use crate::apis::configuration::Configuration;
use std::fs;

pub fn get_user_configuration() -> Configuration {
    let mut configuration = Configuration::new();

    let contents = Some(
        fs::read_to_string("/home/roastbeefer/.config/space-traders/key.txt")
            .expect("Should have been able to read the file")
            .strip_suffix('\n')
            .unwrap()
            .to_string(),
    );

    configuration.bearer_access_token = contents;
    configuration
}
