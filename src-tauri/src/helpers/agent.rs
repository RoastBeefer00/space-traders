use crate::apis::agents_api::*;
use crate::models::*;
use crate::helpers::configuration::get_user_configuration;

#[tauri::command]
pub async fn get_user_agent() -> Agent {
    let configuration = get_user_configuration();

    let my_agent_box = get_my_agent(&configuration).await.unwrap().data;
    let my_agent = *my_agent_box;

    my_agent
}
