use crate::config::load_or_create_config;
use bevy::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Component)]
pub struct FrameAnimation {
  pub frames: Vec<Handle<Image>>,
  pub current: usize,
  pub timer: Timer,
}

impl FrameAnimation {
  pub fn set_fps(&mut self, fps: u32) {
    let duration = std::time::Duration::from_millis(1000 / fps as u64);
    self.timer = Timer::new(duration, TimerMode::Repeating);
  }
}

struct FrameInfo {
  path: PathBuf,
  count: usize,
  fps: u32,
  digits: u32,
  width: f32,
  height: f32,
}

fn check_external_frames() -> Option<FrameInfo> {
  if let Ok(exe_path) = env::current_exe() {
    if let Some(exe_dir) = exe_path.parent() {
      let assets_path = exe_dir.join("assets").join("frames");

      if assets_path.exists() && assets_path.is_dir() {
        if let Ok(entries) = fs::read_dir(&assets_path) {
          let png_count = entries
            .filter_map(|e| e.ok())
            .filter(|e| {
              e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("png"))
                .unwrap_or(false)
            })
            .count();

          if png_count > 0 {
            let config = load_or_create_config();

            return Some(FrameInfo {
              path: assets_path,
              count: png_count,
              fps: config.fps,
              digits: config.frame_digits,
              width: config.frame_width,
              height: config.frame_height,
            });
          }
        }
      }
    }
  }
  None
}

pub fn setup_animation(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2dBundle::default());

  let mut frames = Vec::new();
  let fps: u32;
  let frame_count: usize;

  if let Some(frame_info) = check_external_frames() {
    info!(
      "✨ Loading {} custom frames from: {:?}",
      frame_info.count, frame_info.path
    );
    info!("⚙️ FPS from config: {}", frame_info.fps);
    info!("🔢 Frame digits: {}", frame_info.digits);
    info!("📐 Frame size: {}x{}", frame_info.width, frame_info.height);

    fps = frame_info.fps;
    frame_count = frame_info.count;

    for i in 1..=frame_count {
      let path = format!(
        "frames/frame_{:0width$}.png",
        i,
        width = frame_info.digits as usize
      );
      frames.push(asset_server.load::<Image>(path));
    }
  } else {
    info!("📦 Loading embedded frames");

    let config = load_or_create_config();
    fps = config.fps;
    frame_count = 620;

    info!("⚙️ FPS from config: {}", fps);
    info!(
      "📐 Frame size: {}x{}",
      config.frame_width, config.frame_height
    );

    for i in 1..=frame_count {
      let path = format!("embedded://frames/frame_{:04}.png", i);
      frames.push(asset_server.load::<Image>(path));
    }
  }

  assert!(!frames.is_empty(), "❌ No frames found in assets/frames/");

  let frame_duration = Duration::from_millis(1000 / fps as u64);
  info!(
    "🎬 Animation: {} frames @ {} FPS ({:?} per frame)",
    frame_count, fps, frame_duration
  );

  commands.spawn((
    SpriteBundle {
      texture: frames[0].clone(),
      transform: Transform::from_scale(Vec3::splat(1.0)),
      ..default()
    },
    FrameAnimation {
      frames,
      current: 0,
      timer: Timer::new(frame_duration, TimerMode::Repeating),
    },
  ));
}

use crate::window::AnimControl;

pub fn animate_frames(
  time: Res<Time>,
  anim_ctl: Res<AnimControl>,
  mut q: Query<(&mut FrameAnimation, &mut Handle<Image>)>,
) {
  if anim_ctl.paused || anim_ctl.hidden {
    return;
  }
  for (mut anim, mut tex) in &mut q {
    anim.timer.tick(time.delta());
    if anim.timer.just_finished() {
      anim.current = (anim.current + 1) % anim.frames.len();
      *tex = anim.frames[anim.current].clone();
    }
  }
}
