// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Local, Utc};
use openapi::apis::agents_api::{get_my_agent, *};
use openapi::apis::configuration;
use openapi::apis::configuration::Configuration;
use openapi::apis::contracts_api::get_contracts;
use openapi::apis::fleet_api::get_my_ships;
use openapi::models::*;
use std::fs;

fn get_user_configuration() -> Configuration {
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

#[tauri::command]
async fn get_user_agent() -> Agent {
    let configuration = get_user_configuration();

    let my_agent_box = get_my_agent(&configuration).await.unwrap().data;
    let my_agent = *my_agent_box;

    my_agent
}

#[tauri::command]
async fn get_user_ships() -> Vec<Ship> {
    let configuration = get_user_configuration();

    let ships = get_my_ships(&configuration, None, None).await.unwrap().data;
    ships
}

#[tauri::command]
async fn get_user_contracts() -> Vec<Contract> {
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_user_agent,
            get_user_contracts,
            get_user_ships,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
