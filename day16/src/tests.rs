#[cfg(test)]
mod tests {
  use crate::fft::FFT;

  #[test]
  fn test_fft_single_phase() {
    let mut fft = FFT::from_string("12345678");
    fft.phase();
    assert_eq!(fft.to_string(), "48226158");
  }

  #[test]
  fn test_fft_multiple_phases() {
    let mut fft = FFT::from_string("12345678");
    for _ in 0..4 {
      fft.phase();
    }
    assert_eq!(fft.to_string(), "01029498");
  }

  #[test]
  fn test_fft_example1() {
    let mut fft = FFT::from_string("80871224585914546619083218645595");
    for _ in 0..100 {
      fft.phase();
    }
    assert_eq!(&fft.to_string()[0..8], "24176176");
  }

  #[test]
  fn test_fft_example2() {
    let mut fft = FFT::from_string("19617804207202209144916044189917");
    for _ in 0..100 {
      fft.phase();
    }
    assert_eq!(&fft.to_string()[0..8], "73745418");
  }

  #[test]
  fn test_fft_example3() {
    let mut fft = FFT::from_string("69317163492948606335995924319873");
    for _ in 0..100 {
      fft.phase();
    }
    assert_eq!(&fft.to_string()[0..8], "52432133");
  }

  #[test]
  fn test_fft_part2_example1() {
    let mut fft = FFT::from_string(&"03036732577212944063491565474664".repeat(10000));
    for _ in 0..100 {
      fft.fast_phase();
    }
    assert_eq!(&fft.digest(), "84462026");
  }

  #[test]
  fn test_fft_part2_example2() {
    let mut fft = FFT::from_string(&"02935109699940807407585447034323".repeat(10000));
    for _ in 0..100 {
      fft.fast_phase();
    }
    assert_eq!(&fft.digest(), "78725270");
  }

  #[test]
  fn test_fft_part2_example3() {
    let mut fft = FFT::from_string(&"03081770884921959731165446850517".repeat(10000));
    for _ in 0..100 {
      fft.fast_phase();
    }
    assert_eq!(&fft.digest(), "53553731");
  }
}
