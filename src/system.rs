use bevy::prelude::*;

use crate::state::AnimControl;

pub fn snap_to_taskbar_on_startup(
  mut q_win: Query<&mut bevy::window::Window, With<bevy::window::PrimaryWindow>>,
) {
  if let Ok(mut w) = q_win.get_single_mut() {
    if let Some(info) = crate::taskbar::get_taskbar_info() {
      let win_size = (
        w.resolution.physical_width() as i32,
        w.resolution.physical_height() as i32,
      );
      let (x, y) = crate::taskbar::calc_anchor_on_taskbar(info, win_size, 6, "center");
      w.position = bevy::window::WindowPosition::At(bevy::math::IVec2::new(x, y));
    }
  }
}

pub fn snap_to_taskbar_runtime(
  anim_ctl: Res<AnimControl>,
  mut q_win: Query<&mut bevy::window::Window, With<bevy::window::PrimaryWindow>>,
) {
  if !anim_ctl.follow_taskbar {
    return;
  }

  if let Ok(mut w) = q_win.get_single_mut() {
    if let Some(info) = crate::taskbar::get_taskbar_info() {
      let win_size = (
        w.resolution.physical_width() as i32,
        w.resolution.physical_height() as i32,
      );
      let (x, y) = crate::taskbar::calc_anchor_on_taskbar(info, win_size, 6, "center");
      w.position = bevy::window::WindowPosition::At(bevy::math::IVec2::new(x, y));
    }
  }
}
