use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;
use num::iter::Range;

use crate::common::*;

#[derive(Debug)]
struct Coord {
  x: i64,
  y: i64,
}

impl Coord {
  fn new(x: i64, y: i64) -> Self {
    Coord { x, y }
  }

  fn manhattan(&self, other: &Coord) -> i64 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }
}

fn adjustment_map(
  set: &BTreeSet<usize>,
  value: usize,
) -> Vec<(std::ops::Range<usize>, usize)> {
  set
    .iter()
    .cloned()
    .collect::<Vec<_>>()
    .windows(2)
    .enumerate()
    .map(|(i, w)| (w[0]..w[1], (i + 1) * (value - 1)))
    .collect()
}

fn adjust(map: &[(std::ops::Range<usize>, usize)], idx: usize) -> usize {
  map
    .iter()
    .find(|(range, _)| range.contains(&idx))
    .map(|(_, v)| idx + *v)
    .unwrap_or(idx)
}

#[derive(Debug)]
struct Stars {
  stars: Vec<Coord>,
  stars_adjusted: Vec<Coord>,
}

impl Stars {
  fn from_grid_input(grid: &GridInput, multiplier: usize) -> Self {
    let mut empty_column_idxs: BTreeSet<usize> = (0..grid.width + 1).collect();
    let mut empty_row_idxs: BTreeSet<usize> = (0..grid.height + 1).collect();
    let mut stars = Vec::new();

    for (x, y, c) in grid.iter_with_coords() {
      if c == '#' {
        stars.push(Coord::new(x as i64, y as i64));
        empty_column_idxs.remove(&x);
        empty_row_idxs.remove(&y);
      }
    }

    let column_map = adjustment_map(&empty_column_idxs, multiplier);
    let row_map = adjustment_map(&empty_row_idxs, multiplier);

    let mut stars_adjusted = Vec::new();
    for star in &stars {
      stars_adjusted.push(Coord::new(
        adjust(&column_map, star.x as usize) as i64,
        adjust(&row_map, star.y as usize) as i64,
      ));
    }

    Self {
      stars,
      stars_adjusted,
    }
  }

  fn distance_sum(&self) -> i64 {
    self
      .stars_adjusted
      .iter()
      .combinations(2)
      .map(|xs| xs[0].manhattan(xs[1]))
      .sum()
  }
}

pub fn part_1(input: &str) -> i64 {
  let starfield = Stars::from_grid_input(&GridInput::from_str(input), 2);
  starfield.distance_sum()
}

pub fn part_2(input: &str) -> i64 {
  let starfield =
    Stars::from_grid_input(&GridInput::from_str(input), 1_000_000);
  starfield.distance_sum()
}
