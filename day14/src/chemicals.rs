use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt;
use std::str;

#[derive(Clone, Eq, PartialEq)]
pub struct Formula {
  pub elements: HashMap<String, isize>,
}

impl Formula {
  pub fn new() -> Formula {
    Formula {
      elements: HashMap::new(),
    }
  }

  pub fn get(&self, chemical: &str) -> isize {
    match self.elements.get(chemical) {
      Some(n) => *n,
      None => 0,
    }
  }

  pub fn len(&self) -> usize {
    self.elements.len()
  }

  pub fn pop(&mut self) -> Option<(String, isize)> {
    let first = self.elements.keys().nth(0)?;
    let key = String::from(first);
    let value = self.elements.remove(&key)?;
    Some((key, value))
  }

  pub fn iter(&self) -> Iter<String, isize> {
    self.elements.iter()
  }

  pub fn add(&mut self, chemical: &str, quantity: isize) {
    if quantity == 0 {
      return;
    }

    if self.elements.contains_key(chemical) {
      let new_value = self.get(chemical) + quantity;

      if new_value == 0 {
        self.elements.remove(chemical);
      } else {
        let current = self.elements.get_mut(chemical).unwrap();
        *current = new_value;
      }
    } else {
      self.elements.insert(String::from(chemical), quantity);
    }
  }

  pub fn remove(&mut self, chemical: &str, quantity: isize) {
    self.add(chemical, -quantity)
  }
}
impl fmt::Debug for Formula {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, (chemical, quantity)) in self.elements.iter().enumerate() {
      if i == 0 {
        write!(f, "{} {}", quantity, chemical).unwrap()
      } else {
        write!(f, ", {} {}", quantity, chemical).unwrap()
      }
    }

    write!(f, "")
  }
}
impl str::FromStr for Formula {
  type Err = fmt::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut elements: HashMap<String, isize> = HashMap::new();

    for chemicals in s.split(',') {
      let parts: Vec<&str> = chemicals.trim().split(' ').collect();
      if parts.len() != 2 {
        return Result::Err(fmt::Error);
      }
      let quantity: isize = parts[0].trim().parse().expect("invalid number format");
      let name = String::from(parts[1].trim());

      elements.insert(name, quantity);
    }

    Ok(Formula { elements })
  }
}

pub struct Reaction {
  pub input: Formula,
  pub output: Formula,
}
impl fmt::Debug for Reaction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?} => {:?}", self.input, self.output)
  }
}
impl str::FromStr for Reaction {
  type Err = fmt::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let terms: Vec<&str> = s.split("=>").collect();
    if terms.len() != 2 {
      return Result::Err(fmt::Error);
    }

    Ok(Reaction {
      input: terms[0].trim().parse().expect("invalid input formula"),
      output: terms[1].trim().parse().expect("invalid output formula"),
    })
  }
}
impl Reaction {
  pub fn produces(&self, output: &str) -> bool {
    self.output.get(output) > 0
  }
}
