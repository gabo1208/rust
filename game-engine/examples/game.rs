use game_engine::*;

fn main() {
  set_event_handler(move |key: Key| match key {
      Key::Left => clear_screen_to_color(1.0, 0.0, 0.0, 1.0),
      Key::Right => clear_screen_to_color(0.0, 1.0, 0.0, 1.0),
      Key::Up => clear_screen_to_color(0.0, 0.0, 1.0, 1.0),
      Key::Down => clear_screen_to_color(0.0, 1.0, 1.0, 1.0),
      Key::Space => clear_screen_to_color(1.0, 1.0, 0.0, 1.0),
  });
}
