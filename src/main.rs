#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::{WindowLevel, WindowPosition};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use crossbeam_channel::unbounded;

mod anim;
mod drag;
mod hotkey;
mod state;
mod system;
mod taskbar;

use anim::{animate_frames, setup_animation};
use drag::drag_move_window;
use state::{keep_always_on_top, pump_tray_commands, AnimControl, TrayRx};

#[cfg(target_os = "windows")]
use system::snap_to_taskbar_on_startup;

use crate::system::snap_to_taskbar_runtime;

fn main() {
  let (tx, rx) = unbounded();
  hotkey::spawn_hotkey(tx);

  App::new()
    .add_plugins(EmbeddedAssetPlugin::default())
    .insert_resource(ClearColor(Color::NONE))
    .insert_resource(AnimControl {
      window_pos: IVec2::new(800, 900),
      follow_taskbar: true,
      ..Default::default()
    })
    .insert_resource(TrayRx(rx))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Arisu".into(),
        transparent: true,
        decorations: false,
        resizable: false,
        window_level: WindowLevel::AlwaysOnTop,
        position: WindowPosition::At(IVec2::new(800, 920)),
        resolution: (128.0, 128.0).into(),
        ..default()
      }),
      ..default()
    }))
    .add_systems(Startup, (setup_animation, snap_to_taskbar_on_startup))
    .add_systems(
      Update,
      (
        pump_tray_commands,
        animate_frames,
        drag_move_window,
        keep_always_on_top,
        snap_to_taskbar_runtime,
      ),
    )
    .run();
}
