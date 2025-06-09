use std::fs;
use serde::{Serialize, Deserialize};
use crate::utils::env;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
  pub all_monitors: Option<bool>,
  pub monitors: Option<Vec<String>>
}

pub fn parse_config () -> Config {
  let path_to_config = format!("{}/{}", env::get_home_env(), ".config/gtk-widgets/config.yaml");
  let config_file_content = fs::read_to_string(path_to_config).expect("Could not find config file");

  return serde_yaml::from_str::<Config>(&config_file_content).expect("Could not parse config file");
}
