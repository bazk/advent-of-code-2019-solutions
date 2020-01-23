use minifb::{Key, Window, WindowOptions};

pub struct UI {
  window: Window,
  width: usize,
  height: usize,
  buffer: Vec<u32>,
}

#[allow(dead_code)]
impl UI {
  pub fn new(title: &str, width: usize, height: usize) -> UI {
    let buffer = vec![0; width * height];

    let mut window =
      Window::new(title, width, height, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
      });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    UI {
      window,
      width,
      height,
      buffer,
    }
  }

  pub fn get_key(&self) -> Option<Key> {
    if self.window.is_key_down(Key::A) {
      return Option::Some(Key::A);
    } else if self.window.is_key_down(Key::D) {
      return Option::Some(Key::D);
    } else if self.window.is_key_down(Key::S) {
      return Option::Some(Key::S);
    } else if self.window.is_key_down(Key::A) {
      return Option::Some(Key::A);
    } else {
      return Option::None;
    }
  }

  pub fn clear(&mut self) {
    for p in self.buffer.iter_mut() {
      *p = 0;
    }
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
    self.buffer[y * self.width + x] = color;
  }

  pub fn update(&mut self) -> bool {
    self
      .window
      .update_with_buffer(&self.buffer, self.width, self.height)
      .unwrap();

    self.window.is_open() && !self.window.is_key_down(Key::Escape)
  }
}
