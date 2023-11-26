use crate::apis::fleet_api::get_my_ships;
use crate::models::*;
use crate::helpers::configuration::get_user_configuration;

#[tauri::command]
pub async fn get_user_ships() -> Vec<Ship> {
    let configuration = get_user_configuration();

    let ships = get_my_ships(&configuration, None, None).await.unwrap().data;
    ships
}
