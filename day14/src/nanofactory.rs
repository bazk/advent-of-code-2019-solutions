use crate::chemicals::{Formula, Reaction};
use std::fs;

pub struct NanoFactory {
  reactions: Vec<Reaction>,
}

impl NanoFactory {
  pub fn load(filename: &str) -> NanoFactory {
    let raw = fs::read_to_string(filename).expect("Failed to read input file");
    Self::load_string(&raw)
  }

  pub fn load_string(raw: &str) -> NanoFactory {
    let mut reactions = Vec::new();
    for line in raw.split('\n') {
      let reaction: Reaction = line.trim().parse().expect("invalid reaction format");
      reactions.push(reaction);
    }

    NanoFactory { reactions }
  }

  pub fn minimum_requirements(&self, target: &str, production: isize) -> isize {
    let mut requirements = Formula::new();
    let mut extra = Formula::new();
    let mut total_ore = 0;

    requirements.add(target, production);

    while requirements.len() > 0 {
      let (chemical, quantity) = requirements.pop().unwrap();

      if chemical == "ORE" {
        total_ore += quantity;
        continue;
      }

      let reaction = self
        .reactions
        .iter()
        .find(|reaction| reaction.produces(&chemical))
        .unwrap();

      let output_quantity = reaction.output.get(&chemical);
      let multiplier = (quantity as f64 / output_quantity as f64).ceil() as isize;

      for (input_chemical, &input_quantity) in reaction.input.iter() {
        let req = input_quantity * multiplier;

        if extra.get(input_chemical) > req {
          extra.remove(input_chemical, req);
        } else if extra.get(input_chemical) > 0 {
          let extra_amount = extra.get(input_chemical);
          requirements.add(input_chemical, req - extra_amount);
          extra.remove(input_chemical, extra_amount);
        } else {
          requirements.add(input_chemical, req);
        }
      }

      let remainder = (multiplier * output_quantity) - quantity;
      if remainder > 0 {
        extra.add(&chemical, remainder);
      }
    }

    total_ore
  }

  pub fn maximum_production(&self, target: &str, source_quantity: isize) -> isize {
    let mut low = source_quantity / self.minimum_requirements(target, 1);
    let mut high = low * 2;

    loop {
      if low == high || low + 1 == high {
        return low;
      }

      let current = (low + high) / 2;
      let value = self.minimum_requirements(target, current);

      if value > source_quantity {
        high = current;
      } else {
        low = current;
      }
    }
  }
}
