const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

pub struct FFT {
  value: Vec<isize>,
  offset: usize,
}

#[allow(dead_code)]
impl FFT {
  pub fn from_string(input: &str) -> FFT {
    let value: Vec<isize> = input
      .chars()
      .map(|c: char| c.to_digit(10).expect("unable to parse input") as isize)
      .collect();

    let offset: usize = input[0..7]
      .parse()
      .expect("failed to parse message index from the first 7 digits");

    FFT { value, offset }
  }

  pub fn to_string(&self) -> String {
    self
      .value
      .iter()
      .map(|&n| std::char::from_digit(n as u32, 10).unwrap())
      .collect::<String>()
  }

  pub fn digest(&self) -> String {
    let start = self.offset % self.value.len();
    let end = start + 8;

    self.value[start..end]
      .iter()
      .map(|&n| std::char::from_digit(n as u32, 10).unwrap())
      .collect::<String>()
  }

  pub fn pattern(row: usize, col: usize) -> isize {
    let n = ((col + 1) as f64 / (row + 1) as f64).floor() as usize;
    BASE_PATTERN[n % 4]
  }

  pub fn phase(&mut self) {
    let mut output: Vec<isize> = vec![0; self.value.len()];

    for row in 0..self.value.len() {
      for col in 0..self.value.len() {
        output[row] += self.value[col] * FFT::pattern(row, col);
      }
    }

    for (o, out) in output.iter().enumerate() {
      self.value[o] = out.abs() % 10;
    }
  }

  pub fn fast_phase(&mut self) {
    for col in (self.offset..(self.value.len() - 1)).rev() {
      self.value[col] = (self.value[col] + self.value[col + 1]).abs() % 10;
    }
  }
}
