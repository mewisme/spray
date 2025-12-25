use bevy::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::config::{load_or_create_config, SprayConfig};

#[derive(Resource)]
pub struct ConfigResource {
  pub config: SprayConfig,
  pub changed: bool,
  pub last_modified: Option<SystemTime>,
}

pub fn setup_config_watcher(mut commands: Commands) {
  let config = load_or_create_config();
  let last_modified = get_config_path()
    .and_then(|path| fs::metadata(path).ok())
    .and_then(|meta| meta.modified().ok());

  commands.insert_resource(ConfigResource {
    config: config.clone(),
    changed: false,
    last_modified,
  });

  info!("🔍 Config file watcher started");
}

pub fn watch_config_changes(mut config_res: ResMut<ConfigResource>) {
  if let Some(config_path) = get_config_path() {
    if let Ok(metadata) = fs::metadata(&config_path) {
      if let Ok(modified) = metadata.modified() {
        if let Some(last_modified) = config_res.last_modified {
          if modified > last_modified {
            info!("🔄 Config file changed, reloading...");
            let new_config = load_or_create_config();
            config_res.config = new_config;
            config_res.changed = true;
            config_res.last_modified = Some(modified);
          }
        } else {
          config_res.last_modified = Some(modified);
        }
      }
    }
  }
}

fn get_config_path() -> Option<PathBuf> {
  if let Ok(exe_path) = env::current_exe() {
    if let Some(exe_dir) = exe_path.parent() {
      return Some(exe_dir.join("spray.config.json"));
    }
  }
  None
}

