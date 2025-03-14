use game_engine::*;

fn main() {

  let mut x_position = 200.0;
  let mut y_position = 30.0;

  let mut x_direction = 1.0;
  let mut y_direction = 1.0;

  let speed = 5.0;
  let move_amount = 20.0;

  set_event_handler(move |context, event| match event {
    Event::KeyDown(Key::Left) => x_position -= move_amount,
    Event::KeyDown(Key::Right) => x_position += move_amount,
    Event::KeyDown(Key::Up) => y_position += move_amount,
    Event::KeyDown(Key::Down) => y_position -= move_amount,
    Event::KeyDown(Key::Space) => {}
    Event::Draw => {
      x_position += x_direction * speed;
      y_position += y_direction * speed;
      // Change the horizontal direction if the cube's too far to the left or right.
      if x_position <= 0.0 || x_position >= 500.0 {
        x_direction *= -1.0;
      }
      // Change the vertical direction if the cube's too far to the top or bottom.
      if y_position <= 0.0 || y_position >= 500.0 {
        y_direction *= -1.0;
      }

      context.clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
      context.draw_rectangle(x_position, y_position, 100., 100.);
    }
  })
}
