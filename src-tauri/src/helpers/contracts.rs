use chrono::{DateTime, Local, Utc};
use crate::helpers::configuration::get_user_configuration;
use crate::apis::contracts_api::get_contracts;
use crate::models::*;

#[tauri::command]
pub async fn get_user_contracts() -> Vec<Contract> {
    let configuration = get_user_configuration();

    let mut contracts = get_contracts(&configuration, None, None)
        .await
        .unwrap()
        .data;

    for contract in &mut contracts {
        let utc_time: DateTime<Utc> = contract.terms.deadline.parse().unwrap();
        let local_time: DateTime<Local> = DateTime::from(utc_time);

        contract.terms.deadline = local_time.format("%I:%M:%S%p %Y-%m-%d").to_string();
    }

    contracts
}
