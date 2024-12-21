use std::{collections::BTreeSet, ops::Range};
use take_until::TakeUntilExt;

use nom::{
  bytes::complete::{is_not, tag},
  character::complete::{digit1, line_ending, multispace0, space0},
  combinator::{map_res, opt},
  multi::many1,
  sequence::{preceded, terminated, tuple},
  IResult,
};

use super::range::{RangeMapping, ValueRange};
use crate::parser::ws_line;

#[derive(Debug)]
pub struct Map<'a> {
  _name: &'a str,
  mappings: BTreeSet<RangeMapping>,
}

impl Map<'_> {
  pub fn new(name: &str, ranges: Vec<Vec<i64>>) -> Map {
    let mappings = match ranges.len() {
      0 => BTreeSet::from([RangeMapping::new(0..i64::MAX, 0)]),
      len @ _ => {
        let mut mappings = match len {
          1 => BTreeSet::from([RangeMapping::from_vec(&ranges[0])]),
          _ => ranges
            .windows(2)
            .map(|window| {
              [
                RangeMapping::from_vec(&window[0]),
                RangeMapping::from_vec(&window[1]),
              ]
            })
            .fold(BTreeSet::new(), |mut mappings, [left, right]| {
              if right.range.start > left.range.end {
                mappings.insert(RangeMapping::new(
                  left.range.end..right.range.start,
                  0,
                ));
              }
              mappings.insert(left);
              mappings.insert(right);
              mappings
            }),
        };
        #[allow(irrefutable_let_patterns)]
        if let first = mappings.first().unwrap() {
          if first.range.start > 0 {
            mappings.insert(RangeMapping::new(0..first.range.start, 0));
          }
        }
        #[allow(irrefutable_let_patterns)]
        if let last = mappings.last().unwrap() {
          if last.range.end < i64::MAX {
            mappings.insert(RangeMapping::new(last.range.end..i64::MAX, 0));
          }
        }
        mappings
      }
    };

    Map {
      _name: name,
      mappings,
    }
  }

  pub fn map(&self, value: i64) -> i64 {
    for mapping in &self.mappings {
      if mapping.range.contains(value) {
        return value + mapping.offset;
      }
    }
    value
  }

  pub fn map_range(&self, range: ValueRange) -> Vec<ValueRange> {
    let range_mappings = take_between(
      self.mappings.iter(),
      move |mapping| mapping.range.contains(range.start),
      move |mapping| mapping.range.contains(range.end - 1),
    )
    .collect::<Vec<_>>();

    let len = range_mappings.len();
    if len == 1 {
      vec![range_mappings[0].apply_single(range)]
    } else {
      range_mappings
        .into_iter()
        .enumerate()
        .map(|(i, mapping)| match i {
          0 => mapping.apply_first(range),
          last if last == len - 1 => mapping.apply_last(range),
          _ => mapping.apply_middle(range),
        })
        .collect()
    }
  }

  pub fn map_ranges(&self, ranges: Vec<ValueRange>) -> Vec<ValueRange> {
    ranges
      .into_iter()
      .flat_map(|range| self.map_range(range))
      .collect()
  }
}

#[derive(Debug)]
pub struct Almanac<'a> {
  pub seeds: Vec<ValueRange>,
  pub maps: Vec<Map<'a>>,
}

impl Almanac<'_> {
  pub fn new(seeds: Vec<ValueRange>, maps: Vec<Map>) -> Almanac {
    Almanac { seeds, maps }
  }

  pub fn map(&self, value: i64) -> i64 {
    self.maps.iter().fold(value, |acc, map| map.map(acc))
  }

  pub fn map_ranges(&self, ranges: Vec<ValueRange>) -> Vec<ValueRange> {
    self
      .maps
      .iter()
      .fold(ranges, move |acc, map| map.map_ranges(acc))
  }

  pub fn map_ranges_lowest(&self) -> i64 {
    self
      .map_ranges(self.seeds.clone())
      .iter()
      .map(|range| range.start)
      .min()
      .unwrap()
  }

  pub fn map_seeds(&self) -> impl Iterator<Item = i64> + '_ {
    self
      .seeds
      .iter()
      .flat_map(|range| range.iter().map(|seed| self.map(seed)))
  }

  pub fn map_seeds_lowest(&self) -> i64 {
    self.map_seeds().min().unwrap()
  }
}

fn number_list(input: &str) -> IResult<&str, Vec<i64>> {
  terminated(
    many1(map_res(terminated(digit1, space0), |s: &str| {
      s.parse::<i64>()
    })),
    opt(line_ending),
  )(input)
}

fn number_lists(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
  terminated(many1(number_list), multispace0)(input)
}

fn seeds(input: &str) -> IResult<&str, Vec<i64>> {
  terminated(preceded(ws_line(tag("seeds:")), number_list), multispace0)(input)
}

fn map_name(input: &str) -> IResult<&str, &str> {
  terminated(is_not(" "), tuple((ws_line(tag("map:")), line_ending)))(input)
}

fn map(input: &str) -> IResult<&str, Map> {
  let (input, (name, lists)) = tuple((map_name, number_lists))(input)?;
  Ok((input, Map::new(name, lists)))
}

fn maps(input: &str) -> IResult<&str, Vec<Map>> {
  many1(map)(input)
}

pub fn parser_list(input: &str) -> IResult<&str, Almanac> {
  let (input, seeds) = seeds(input)?;
  let (input, maps) = maps(input)?;
  let rangelist = seeds
    .into_iter()
    .map(|seed| ValueRange::new(seed, seed + 1))
    .collect();
  Ok((input, Almanac::new(rangelist, maps)))
}

pub fn parser_ranges(input: &str) -> IResult<&str, Almanac> {
  let (input, seeds) = seeds(input)?;
  let (input, maps) = maps(input)?;
  let rangelist = seeds
    .chunks(2)
    .map(|chunks| ValueRange::new(chunks[0], chunks[0] + chunks[1]))
    .collect();
  Ok((input, Almanac::new(rangelist, maps)))
}

pub fn take_between<T, I, P1, P2>(
  iter: I,
  mut mark_start: P1,
  mut mark_end: P2,
) -> impl Iterator<Item = T>
where
  T: Ord,
  I: Iterator<Item = T>,
  P1: FnMut(&T) -> bool,
  P2: FnMut(&T) -> bool,
{
  iter
    .skip_while(move |item| !mark_start(item))
    .take_until(move |item| mark_end(item))
}
