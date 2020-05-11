use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

static NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node {
  id: usize,
  children: Vec<Edge>,
  label: String,
}

impl Node {
  pub fn new(label: String) -> Self {
    Self {
      id: NODE_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
      children: Vec::new(),
      label,
    }
  }
}

impl fmt::Display for Node {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for edge in self.children.iter() {
      write!(
        f,
        "  {} -> {} [label=\"[{},{}]\"]\n",
        self.id, edge.to.id, edge.label.0, edge.label.1
      )
      .unwrap();

      // recursively format the child nodes
      write!(f, "{}", edge.to).unwrap();
    }

    let is_leaf = self.children.len() == 0;
    let shape = if is_leaf { "box" } else { "circle" };
    write!(
      f,
      "{} [label=\"{}\", shape=\"{}\"]\n",
      self.id, self.label, shape
    )
    .unwrap();

    write!(f, "")
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Edge {
  to: Node,
  label: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct SuffixTree<'a, T> {
  root: Node,
  data: &'a Vec<T>,
}

impl<'a, T> fmt::Display for SuffixTree<'a, T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "digraph SuffixTree {{\n").unwrap();

    write!(f, "{}", self.root).unwrap();

    write!(f, "}}\n").unwrap();

    write!(f, "")
  }
}

#[allow(dead_code)]
impl<'a, T> SuffixTree<'a, T>
where
  T: Clone + PartialEq + std::fmt::Debug,
{
  pub fn new(data: &'a Vec<T>) -> Self {
    let n = data.len();

    let mut root = Node::new(String::new());

    for i in 0..n {
      match Self::find_by_prefix(&data, &mut root, (i, n)) {
        Some(edge) => {
          let (a, b) = edge.label;

          let prefix = (a, a + (n - i));
          let suffix = (a + (n - i) + 1, b);

          let mut node = Node::new(String::new());

          node.children.push(Edge {
            to: edge.to,
            label: suffix,
          });
          root.children.push(Edge {
            to: node,
            label: prefix,
          });
        }
        None => {
          root.children.push(Edge {
            to: Node::new(format!("{}", i)),
            label: (i, n),
          });
        }
      }
    }

    Self { root, data }
  }

  pub fn patterns(&self) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    for edge in self.root.children.iter() {
      if edge.to.children.len() == 0 {
        continue;
      }

      res.push(edge.label);
    }

    res
  }

  fn find_by_prefix(
    data: &Vec<T>,
    node: &mut Node,
    (pre_begin, pre_end): (usize, usize),
  ) -> Option<Edge> {
    let found = node.children.iter().position(|edge| {
      let (edge_begin, edge_end) = edge.label;

      if (edge_end - edge_begin) <= (pre_end - pre_begin) {
        return false;
      }

      let mut m = 0;
      for i in 0..(pre_end - pre_begin) {
        if data[pre_begin + i] != data[edge_begin + i] {
          break;
        }
        m += 1;
      }

      return m == pre_end - pre_begin;
    });

    match found {
      Some(index) => Some(node.children.remove(index)),
      None => None,
    }
  }
}
