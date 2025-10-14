use crate::state::AnimControl;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowPosition};

#[cfg(target_os = "windows")]
fn get_cursor_pos() -> Option<(i32, i32)> {
  use windows::Win32::Foundation::POINT;
  use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

  unsafe {
    let mut point = POINT::default();
    if GetCursorPos(&mut point).is_ok() {
      Some((point.x, point.y))
    } else {
      None
    }
  }
}

#[cfg(not(target_os = "windows"))]
fn get_cursor_pos() -> Option<(i32, i32)> {
  None
}

pub fn drag_move_window(
  mut anim_ctl: ResMut<AnimControl>,
  mut q_win: Query<&mut Window, With<PrimaryWindow>>,
  mouse: Res<ButtonInput<MouseButton>>,
) {
  if let Ok(mut win) = q_win.get_single_mut() {
    if mouse.just_pressed(MouseButton::Left) {
      if win.cursor_position().is_some() {
        if let Some(cursor_pos) = get_cursor_pos() {
          anim_ctl.dragging = true;
          anim_ctl.drag_start_cursor = Some(cursor_pos);
          anim_ctl.drag_start_window = Some(anim_ctl.window_pos);
        }
      }
    }

    if anim_ctl.dragging && mouse.pressed(MouseButton::Left) {
      if let (Some(cursor_pos), Some(start_cursor), Some(start_window)) = (
        get_cursor_pos(),
        anim_ctl.drag_start_cursor,
        anim_ctl.drag_start_window,
      ) {
        let delta_x = cursor_pos.0 - start_cursor.0;
        let delta_y = cursor_pos.1 - start_cursor.1;

        let new_x = start_window.x + delta_x;
        let new_y = start_window.y + delta_y;

        #[cfg(target_os = "windows")]
        let (screen_width, screen_height) = {
          use windows::Win32::UI::WindowsAndMessaging::{
            GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
          };
          unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) }
        };

        #[cfg(not(target_os = "windows"))]
        let (screen_width, screen_height) = (1920, 1080);

        let win_width = win.resolution.physical_width() as i32;
        let win_height = win.resolution.physical_height() as i32;

        let clamped_x = new_x.clamp(0, screen_width - win_width);
        let clamped_y = new_y.clamp(0, screen_height - win_height);

        anim_ctl.window_pos = IVec2::new(clamped_x, clamped_y);
        win.position = WindowPosition::At(anim_ctl.window_pos);
      }
    }

    if mouse.just_released(MouseButton::Left) {
      anim_ctl.dragging = false;
      anim_ctl.drag_start_cursor = None;
      anim_ctl.drag_start_window = None;
    }
  }
}
