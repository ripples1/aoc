mod parser;
use std::collections::HashMap;

use super::d08::parser::parser as graph_parser;

fn take_dir<T>(c: char, tuple: (T, T)) -> T {
  match c {
    'L' => tuple.0,
    'R' => tuple.1,
    _ => panic!("Invalid direction"),
  }
}

fn is_zzz(s: &str) -> bool {
  s == "ZZZ"
}

fn ends_with_z(s: &str) -> bool {
  s.ends_with("Z")
}

fn follow_path(
  edges: &HashMap<&str, (&str, &str)>,
  start: &str,
  end_fn: fn(&str) -> bool,
  path: &str,
) -> i64 {
  let mut count = 0i64;
  let mut current = start;
  for dir in path.chars().cycle() {
    current = take_dir(dir, edges[current]);
    count += 1;
    if end_fn(current) {
      break;
    }
  }
  count
}

#[derive(Debug)]
struct Graph<'a> {
  edges: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Graph<'a> {
  fn from_list(node_list: Vec<(&'a str, (&'a str, &'a str))>) -> Self {
    let edges =
      node_list
        .into_iter()
        .fold(HashMap::new(), |mut g, (name, (a, b))| {
          g.insert(name, (a, b));
          g
        });
    Self { edges }
  }

  fn follow_path(&self, path: &str) -> i64 {
    follow_path(&self.edges, "AAA", is_zzz, path)
  }

  fn follow_path2(&self, path: &str) -> i64 {
    let cycles = self
      .edges
      .keys()
      .filter(|k| k.ends_with("A"))
      .map(|n| follow_path(&self.edges, n, ends_with_z, path))
      .collect::<Vec<_>>();
    let (x, xs) = cycles.split_at(1);
    xs.iter()
      .fold(x[0], |acc, x| num::Integer::lcm(&acc, x))
  }
}

pub fn part_1(input: &str) -> i64 {
  let (path, node_list) = graph_parser(input).unwrap();
  let g = Graph::from_list(node_list);
  g.follow_path(path)
}

pub fn part_2(input: &str) -> i64 {
  let (path, node_list) = graph_parser(input).unwrap();
  let g = Graph::from_list(node_list);
  g.follow_path2(path)
}
