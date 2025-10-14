use crate::state::AnimControl;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowPosition};

pub fn drag_move_window(
  mut anim_ctl: ResMut<AnimControl>,
  mut q_win: Query<&mut Window, With<PrimaryWindow>>,
  mouse: Res<ButtonInput<MouseButton>>,
) {
  if anim_ctl.locked {
    anim_ctl.dragging = false;
    anim_ctl.last_cursor = None;
    return;
  }

  if let Ok(mut win) = q_win.get_single_mut() {
    if let Some(cursor) = win.cursor_position() {
      if mouse.just_pressed(MouseButton::Left) {
        anim_ctl.dragging = true;
        anim_ctl.last_cursor = Some(cursor);
      }
      if anim_ctl.dragging && mouse.pressed(MouseButton::Left) {
        if let Some(last) = anim_ctl.last_cursor {
          let delta = cursor - last;
          anim_ctl.window_pos.x += delta.x.round() as i32;
          anim_ctl.window_pos.y += delta.y.round() as i32;
          win.position = WindowPosition::At(anim_ctl.window_pos);
          anim_ctl.last_cursor = Some(cursor);
        } else {
          anim_ctl.last_cursor = Some(cursor);
        }
      }
      if mouse.just_released(MouseButton::Left) {
        anim_ctl.dragging = false;
        anim_ctl.last_cursor = None;
      }
    } else if mouse.just_released(MouseButton::Left) {
      anim_ctl.dragging = false;
      anim_ctl.last_cursor = None;
    }
  }
}
