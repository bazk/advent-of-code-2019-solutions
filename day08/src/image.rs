use std::fs;
use std::fmt;

pub struct Layer {
    pub pixels: Vec<Vec<u32>>
}

impl Clone for Layer {
    fn clone(&self) -> Layer {
        Layer {
            pixels: self.pixels.clone()
        }
    }
}

impl Layer {
    #[allow(dead_code)]
    pub fn count_pixels(&self, matcher: impl Fn(u32) -> bool) -> u32 {
        self.pixels.iter()
            .map(|row| row.iter().fold(0, |acc, col| {
                if matcher(*col as u32) {
                    acc + 1
                } else {
                    acc
                }
            }))
            .fold(0, |acc, row| acc + row)
    }
}

pub struct Image {
    pub layers: Vec<Layer>,
    pub width: usize,
    pub height: usize
}

impl Clone for Image {
    fn clone(&self) -> Image {
        Image {
            layers: self.layers.clone(),
            width: self.width,
            height: self.height
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let mut color = 2;

                for layer in self.layers.iter() {
                    if layer.pixels[row][col] != 2 {
                        color = layer.pixels[row][col];
                        break;
                    }
                };

                (match color {
                    0 => write!(f, "  "),
                    1 => write!(f, "\u{2588}\u{2588}"),
                    _ => write!(f, "\u{2591}\u{2591}")
                }).unwrap();
            }
            write!(f, "\n").unwrap();
        }

        write!(f, "")
    }
}

impl Image {
    #[allow(dead_code)]
    pub fn open(filename: &str, width: usize, height: usize) -> Image {
        let input = fs::read_to_string(filename).expect("Failed to read image file");

        let mut raw = vec![0; input.len()];
        let mut i = 0;
        for pixel in input.chars() {
            // skip non numeric chars
            if !pixel.is_ascii_digit() {
                continue
            }

            raw[i] = pixel.to_digit(10).expect("Invalid image format");
            i += 1;
        };

        let mut layers = Vec::new();
        let mut i = 0;
        loop {
            let mut layer = Layer {
                pixels: vec![vec![0; width]; height]
            };

            for row in 0..height {
                for col in 0..width {
                    layer.pixels[row][col] = raw[i];
                    i += 1;
                }
            }

            layers.push(layer);

            if i >= raw.len() {
                break
            }
        }

        Image {
            layers,
            width,
            height
        }
    }

    #[allow(dead_code)]
    pub fn checksum(&self) -> u32 {
        let mut minimum_count: u32 = std::u32::MAX;
        let mut minimum_layer: usize = 0;

        for (layer_index, layer) in self.layers.iter().enumerate() {
            let count = layer.count_pixels(|pixel| pixel == 0);

            if count < minimum_count {
                minimum_count = count;
                minimum_layer = layer_index;
            }
        }

        let ones = self.layers[minimum_layer].count_pixels(|pixel| pixel == 1);
        let twos = self.layers[minimum_layer].count_pixels(|pixel| pixel == 2);

        ones * twos
    }
}
