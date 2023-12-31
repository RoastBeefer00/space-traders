use crate::apis::systems_api::*;
use crate::helpers::configuration::get_user_configuration;
use crate::models::*;

#[tauri::command]
pub async fn get_waypoints(
    system: &str,
    waypoint_type: Option<WaypointType>,
    traits: Option<GetSystemWaypointsTraitsParameter>,
) -> Result<Vec<Waypoint>, ()> {
    let configuration = get_user_configuration();

    println!("{:#?}", system.clone());
    println!("{:#?}", waypoint_type);
    let waypoints = get_system_waypoints(&configuration, system, None, None, waypoint_type, traits)
        .await
        .unwrap()
        .data;

    Ok(waypoints)
}
