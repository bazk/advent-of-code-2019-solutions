use crate::path::Step;
use crate::suffix_tree::SuffixTree;

pub fn compress(data: &Vec<Step>) -> Option<Vec<Vec<Step>>> {
  let mut routine = data.clone();

  let tree: SuffixTree<Step> = SuffixTree::new(data);

  for (start, end) in tree.patterns() {
    if (end - start) > 20 {
      continue;
    }

    let a: Vec<Step> = data[start..end].iter().cloned().collect();
    replace_pattern(&mut routine, &a, Step::FunctionCall(String::from("A")));

    let mut parts = missing_parts(&routine);
    if parts.len() > 2 {
      continue;
    }

    let b = if parts.len() > 0 {
      let part = parts.remove(0);
      replace_pattern(&mut routine, &part, Step::FunctionCall(String::from("B")));
      part
    } else {
      Vec::new()
    };

    let c = if parts.len() > 0 {
      let part = parts.remove(0);
      replace_pattern(&mut routine, &part, Step::FunctionCall(String::from("C")));
      part
    } else {
      Vec::new()
    };

    return Some(vec![routine, a, b, c]);
  }

  None
}

fn replace_pattern(routine: &mut Vec<Step>, pattern: &Vec<Step>, replacement: Step) {
  let mut idx = 0;
  let mut start = 0;
  let mut pat = 0;

  loop {
    if idx >= routine.len() {
      break;
    }

    let matching = routine[idx] == pattern[pat];

    if matching {
      pat += 1;
    }

    if pat == pattern.len() - 1 {
      routine.splice(
        start..(start + pattern.len()),
        [replacement.clone()].iter().cloned(),
      );
      idx = start;
      pat = 0;
      start = idx + 1;
    }

    if !matching {
      pat = 0;
      start = idx + 1;
    }

    idx += 1;
  }
}

fn missing_parts(routine: &Vec<Step>) -> Vec<Vec<Step>> {
  let mut ret: Vec<Vec<Step>> = Vec::new();
  let mut cur: Vec<Step> = Vec::new();

  for step in routine.iter() {
    match step {
      Step::FunctionCall(_) => {
        if cur.len() > 0 {
          ret.push(cur);
          cur = Vec::new();
        }
      }
      _ => {
        cur.push(step.clone());
      }
    }
  }

  if cur.len() > 0 {
    ret.push(cur);
  }

  ret
}
