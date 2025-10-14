use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::state::AnimControl;

#[cfg(target_os = "windows")]
pub fn snap_to_taskbar_on_startup(
  mut anim_ctl: ResMut<AnimControl>,
  mut q_win: Query<&mut bevy::window::Window, With<PrimaryWindow>>,
) {
  if let Ok(mut w) = q_win.get_single_mut() {
    if let Some(info) = crate::taskbar::get_taskbar_info() {
      let win_size = (
        w.resolution.physical_width() as i32,
        w.resolution.physical_height() as i32,
      );
      let (x, y) = crate::taskbar::calc_anchor_on_taskbar(info, win_size, 6, "center");
      let pos = bevy::math::IVec2::new(x, y);
      w.position = bevy::window::WindowPosition::At(pos);
      anim_ctl.window_pos = pos;
    }
  }
}

#[cfg(not(target_os = "windows"))]
pub fn snap_to_taskbar_on_startup(
  _anim_ctl: ResMut<AnimControl>,
  _q_win: Query<&mut bevy::window::Window, With<PrimaryWindow>>,
) {
}
