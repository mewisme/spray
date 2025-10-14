use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

use crate::animation::FrameAnimation;
use crate::config::ConfigResource;

pub fn apply_config_changes(
  mut config_res: ResMut<ConfigResource>,
  mut anim_query: Query<&mut FrameAnimation>,
  mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
  if !config_res.changed {
    return;
  }

  info!("✨ Applying new config settings...");

  let config = &config_res.config;

  if let Ok(mut anim) = anim_query.get_single_mut() {
    anim.set_fps(config.fps);
    info!("⚙️ FPS updated to: {}", config.fps);
  }

  if let Ok(mut window) = window_query.get_single_mut() {
    window.resolution = WindowResolution::new(config.frame_width, config.frame_height);
    window.title = config.window_title.clone();
    info!(
      "📐 Window size updated to: {}x{}",
      config.frame_width, config.frame_height
    );
    info!("📝 Window title updated to: {}", config.window_title);
  }

  #[cfg(windows)]
  {
    use crate::config::apply_startup_setting;
    apply_startup_setting(&config);
  }

  config_res.changed = false;
  info!("✅ Config applied successfully");
}
