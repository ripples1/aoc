use self::parser::{parser_list, parser_ranges};

mod parser;
mod range;

pub fn part_1(input: &str) -> i64 {
  let (_, almanac) = parser_list(input).unwrap();
  almanac.map_ranges_lowest()
}

pub fn part_2(input: &str) -> i64 {
  let (_, almanac) = parser_ranges(input).unwrap();
  almanac.map_ranges_lowest()
}
