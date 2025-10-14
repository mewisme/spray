#![allow(dead_code)]
#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy::window::{WindowLevel, WindowPosition};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use std::env;

mod animation;
mod config;
mod platform;
mod window;

use animation::{animate_frames, setup_animation};
use config::{
  apply_config_changes, load_or_create_config, setup_config_watcher, watch_config_changes,
};
use platform::snap_to_taskbar_on_startup;
use window::{drag_move_window, keep_always_on_top, AnimControl};

fn main() {
  if let Ok(exe_path) = env::current_exe() {
    if let Some(exe_dir) = exe_path.parent() {
      let _ = env::set_current_dir(exe_dir);
    }
  }

  let config = load_or_create_config();
  let window_width = config.frame_width;
  let window_height = config.frame_height;

  App::new()
    .add_plugins(EmbeddedAssetPlugin::default())
    .insert_resource(ClearColor(Color::NONE))
    .insert_resource(AnimControl::default())
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Arisu".into(),
        transparent: true,
        decorations: false,
        resizable: false,
        window_level: WindowLevel::AlwaysOnTop,
        position: WindowPosition::At(IVec2::new(800, 920)),
        resolution: (window_width, window_height).into(),
        ..default()
      }),
      ..default()
    }))
    .add_systems(
      Startup,
      (
        setup_config_watcher,
        setup_animation,
        snap_to_taskbar_on_startup,
      )
        .chain(),
    )
    .add_systems(
      Update,
      (
        watch_config_changes,
        apply_config_changes,
        animate_frames,
        drag_move_window,
        keep_always_on_top,
      ),
    )
    .run();
}
