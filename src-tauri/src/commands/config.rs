use tauri::State;

use crate::{error::MaaResult, ConfigHolderState};

#[tauri::command]
pub async fn change_client_type(
    client_type: String,
    config_holder: State<'_, ConfigHolderState>,
) -> MaaResult<()> {
    let mut config_holder = config_holder.lock().await;

    config_holder.write(|config| {
        config.start_up.client_type = match client_type.as_str() {
            "bilibili" => crate::config::start_up::ClientType::Bilibili,
            _ => crate::config::start_up::ClientType::Official,
        };
    })?;

    Ok(())
}
