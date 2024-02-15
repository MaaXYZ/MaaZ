use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;
use tracing::error;

use crate::error::MaaResult;

#[derive(Deserialize, Debug)]
pub struct PluginResource {
    pub name: String,
    pub path: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TaskOption {
    pub name: String,
    pub param: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct PluginTask {
    pub name: String,
    pub entry: String,
    pub option: HashMap<String, Vec<TaskOption>>,
    pub param: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct PluginInterface {
    pub name: String,
    pub resource: Vec<PluginResource>,
    pub task: Vec<PluginTask>,
}

impl<'de> Deserialize<'de> for PluginInterface {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let name = value["name"]
            .as_str()
            .ok_or(serde::de::Error::missing_field("name"))?
            .to_owned();
        let resource: Vec<PluginResource> =
            serde_json::from_value(value["resource"].clone()).map_err(serde::de::Error::custom)?;

        let options = value["option"]
            .as_object()
            .ok_or(serde::de::Error::missing_field("option"))?;
        let options = options
            .iter()
            .map(|(k, v)| {
                let v = v
                    .as_object()
                    .ok_or(serde::de::Error::custom("option value is not an object"))?;
                let v = v["cases"]
                    .as_array()
                    .ok_or(serde::de::Error::missing_field("cases"))?;
                let v = v
                    .iter()
                    .map(|option_v| {
                        let option_v = option_v
                            .as_object()
                            .ok_or(serde::de::Error::custom("option value is not an object"))?;
                        let option_name = option_v["name"]
                            .as_str()
                            .ok_or(serde::de::Error::missing_field("name"))?
                            .to_owned();
                        let param = serde_json::from_value(option_v["param"].clone())
                            .map_err(serde::de::Error::custom)?;
                        Ok(TaskOption {
                            name: option_name,
                            param,
                        })
                    })
                    .collect::<Result<Vec<TaskOption>, _>>()?;
                Ok((k.to_string(), v))
            })
            .collect::<Result<HashMap<String, Vec<TaskOption>>, _>>()?;

        let task = value["task"]
            .as_array()
            .ok_or(serde::de::Error::missing_field("task"))?;
        let task =
            task.iter()
                .map(|v| {
                    let v = v
                        .as_object()
                        .ok_or(serde::de::Error::custom("task value is not an object"))?;
                    let k = v["name"]
                        .as_str()
                        .ok_or(serde::de::Error::missing_field("name"))?;
                    let entry = v["entry"]
                        .as_str()
                        .ok_or(serde::de::Error::missing_field("entry"))?
                        .to_owned();
                    let option = v.get("option");
                    let option = match option {
                        Some(option) => {
                            let option = option
                                .as_array()
                                .ok_or(serde::de::Error::custom("option value is not an array"))?;
                            option.clone()
                        }
                        None => vec![],
                    };
                    let option = option
                        .iter()
                        .map(|option_v| {
                            let option_name = option_v
                                .as_str()
                                .ok_or(serde::de::Error::missing_field("name"))?
                                .to_owned();
                            let option_item = options
                                .get(&option_name)
                                .ok_or(serde::de::Error::custom("option not found"))?;
                            Ok((option_name, option_item.clone()))
                        })
                        .collect::<Result<HashMap<String, Vec<TaskOption>>, _>>()?;
                    let param = v.get("param");
                    let param = match param {
                        Some(param) => serde_json::from_value(param.clone())
                            .map_err(serde::de::Error::custom)?,
                        None => HashMap::new(),
                    };
                    Ok(PluginTask {
                        name: k.to_owned(),
                        entry,
                        option,
                        param,
                    })
                })
                .collect::<Result<Vec<PluginTask>, _>>()?;

        Ok(PluginInterface {
            name,
            resource,
            task,
        })
    }
}

pub fn get_plugins() -> MaaResult<HashMap<String, PluginInterface>> {
    let plugin_dir = std::env::current_exe()
        .map_err(|e| {
            error!("error while getting current executable path: {}", e);
            e
        })?
        .parent()
        .ok_or_else(|| {
            error!("error while getting current executable path");
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "error while getting current executable path",
            )
        })?
        .join("plugins");

    let plugins = std::fs::read_dir(plugin_dir)
        .map_err(|e| {
            error!("error while reading plugin directory: {}", e);
            e
        })?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let is_json = path.extension()?.to_str()? == "json";
            is_json.then_some(path)
        })
        .collect::<Vec<_>>();

    plugins
        .iter()
        .map(|path| {
            let content = std::fs::read_to_string(path).map_err(|e| {
                error!("error while reading plugin file: {}", e);
                e
            })?;
            let interface: PluginInterface = serde_json::from_str(&content).map_err(|e| {
                error!("error while deserializing plugin file: {}", e);
                e
            })?;
            Ok((interface.name.clone(), interface))
        })
        .collect()
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::print_stdout)]
    #![allow(clippy::use_debug)]

    use super::PluginInterface;

    #[test]
    fn test_deserialize() {
        let example_json = include_str!("./test/interface_example.json");
        let interface: Result<PluginInterface, _> = serde_json::from_str(example_json);

        let interface = interface.unwrap();

        println!("{:?}", interface.task);
    }
}
