use serde::Deserialize;
use tauri::State;

use crate::{
    error::{MaaError, MaaResult},
    maa, InstHandle, PluginsState,
};

#[tauri::command]
pub fn get_plugins(state: State<PluginsState>) -> Vec<String> {
    state.keys().cloned().collect()
}

#[derive(Deserialize)]
pub struct LoadPluginParam {
    pub name: String,
    pub resource: Option<String>,
}

#[tauri::command]
pub fn load_plugin(
    state: State<PluginsState>,
    inst: State<InstHandle>,
    param: LoadPluginParam,
) -> MaaResult<()> {
    let plugin = state
        .get(&param.name)
        .ok_or(MaaError::PluginNotExist(param.name.clone()))?;
    if plugin.resource.is_empty() {
        return Err(MaaError::PluginNoResource);
    }

    if plugin.resource.len() == 1 || param.resource.is_none() {
        // load the first resource
        return maa::init_resources(&inst, &plugin.resource[0].path);
    }

    #[allow(clippy::unwrap_used)]
    let resource = param.resource.unwrap();

    let resource = plugin
        .resource
        .iter()
        .find(|r| r.name == resource)
        .ok_or(MaaError::PluginNoResource)?;

    maa::init_resources(&inst, &resource.path)
}
