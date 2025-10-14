use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowLevel, WindowPosition};

#[derive(Resource, Default)]
pub struct AnimControl {
  pub paused: bool,
  pub hidden: bool,
  pub dragging: bool,
  pub drag_start_cursor: Option<(i32, i32)>,
  pub drag_start_window: Option<IVec2>,
  pub window_pos: IVec2,
}

pub fn keep_always_on_top(mut q_win: Query<&mut Window, With<PrimaryWindow>>) {
  if let Ok(mut w) = q_win.get_single_mut() {
    w.window_level = WindowLevel::AlwaysOnTop;
    if let WindowPosition::At(pos) = w.position {
      w.position = WindowPosition::At(pos);
    }
  }
}
