use std::collections::HashMap;

const fn linear_index(row: usize, col: usize, width: usize) -> usize {
  row * width + col
}

type Num = u32;
type NumHashMap = HashMap<usize, Num>;

fn hash_numbers(lines: &Vec<&str>) -> NumHashMap {
  let mut hash = HashMap::new();
  for (i, line) in lines.iter().enumerate() {
    let mut start: Option<usize> = None;
    let mut end = 0;

    fn token_end(
      maybe_start: Option<usize>,
      end: usize,
      line: &str,
      hash: &mut NumHashMap,
      row: usize,
    ) {
      if let Some(start) = maybe_start {
        let value = line[start..end].parse::<Num>().unwrap();
        for col in start..end {
          hash.insert(linear_index(row, col, line.len()), value);
        }
      }
    }

    for (j, c) in line.chars().enumerate() {
      if c.is_ascii_digit() {
        if start.is_none() {
          start = Some(j);
        }
        end = j;
      } else {
        token_end(start, end + 1, line, &mut hash, i);
        start = None;
      }
    }
    token_end(start, end + 1, line, &mut hash, i);
  }
  hash
}

fn is_part_symbol(c: &char) -> bool {
  !(c.is_ascii_digit() || *c == '.')
}

#[derive(Debug)]
struct Part {
  symbol: char,
  values: Vec<Num>,
}

impl Part {
  fn empty(symbol: char) -> Part {
    Part {
      symbol,
      values: Vec::new(),
    }
  }

  fn add_value(&mut self, value: Num) {
    self.values.push(value);
  }

  fn sum(&self) -> Num {
    self.values.iter().sum()
  }

  fn product(&self) -> Num {
    self.values.iter().product()
  }
}

fn extract_parts(lines: &Vec<&str>, hash: &NumHashMap) -> Vec<Part> {
  let mut parts: Vec<Part> = Vec::new();
  for (row, line) in lines.iter().enumerate() {
    for (col, c) in line.chars().enumerate() {
      if !is_part_symbol(&c) {
        continue;
      }
      let mut part = Part::empty(c);
      let width = line.len();
      let irow = row as isize;
      let icol = col as isize;
      for i in [irow - 1, irow, irow + 1].into_iter() {
        // Check middle column first
        for j in [icol, icol - 1, icol + 1].into_iter() {
          // Skip if out of bounds
          if i < 0 || j < 0 || i >= lines.len() as isize || j >= width as isize
          {
            continue;
          }
          // Skip if we are at the symbol itself
          if i == irow && j == icol {
            continue;
          }
          if let Some(value) =
            hash.get(&linear_index(i as usize, j as usize, width))
          {
            part.add_value(*value);
            // If there is a number in the middle, left and right are not possible
            if j == icol {
              break;
            }
          }
        }
      }
      if !part.values.is_empty() {
        parts.push(part);
      }
    }
  }
  parts
}

pub fn part_1(input: &str) -> i64 {
  let lines: Vec<&str> = input.lines().collect();
  let hash = hash_numbers(&lines);
  let parts = extract_parts(&lines, &hash);
  parts.iter().map(|part| part.sum() as i64).sum()
}

pub fn part_2(input: &str) -> i64 {
  let lines: Vec<&str> = input.lines().collect();
  let hash = hash_numbers(&lines);
  let parts = extract_parts(&lines, &hash);
  parts
    .iter()
    .filter(|part| part.symbol == '*' && part.values.len() == 2)
    .map(|part| part.product() as i64)
    .sum()
}
