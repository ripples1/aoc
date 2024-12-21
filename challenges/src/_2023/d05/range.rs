use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Range;

#[derive(Clone, Copy)]
pub struct ValueRange {
  pub start: i64,
  pub end: i64,
}

impl ValueRange {
  pub fn new(start: i64, end: i64) -> ValueRange {
    ValueRange { start, end }
  }

  pub fn iter(&self) -> impl Iterator<Item = i64> {
    self.start..self.end
  }

  pub fn from_range(range: &Range<i64>) -> ValueRange {
    ValueRange::new(range.start, range.end)
  }

  pub fn contains(&self, value: i64) -> bool {
    self.start <= value && value < self.end
  }
}

impl Debug for ValueRange {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}..{}", self.start, self.end)
  }
}

#[derive(Debug)]
pub struct RangeMapping {
  pub range: ValueRange,
  pub offset: i64,
}

impl RangeMapping {
  pub fn new(range: Range<i64>, offset: i64) -> RangeMapping {
    RangeMapping {
      range: ValueRange::from_range(&range),
      offset,
    }
  }

  pub fn from_vec(vec: &Vec<i64>) -> RangeMapping {
    let [dst, src, len] = vec[..] else {
      panic!("Invalid range mapping");
    };
    RangeMapping::new(src..src + len, dst - src)
  }

  pub fn apply_single(&self, range: ValueRange) -> ValueRange {
    ValueRange::new(range.start + self.offset, range.end + self.offset)
  }

  pub fn apply_first(&self, range: ValueRange) -> ValueRange {
    ValueRange::new(range.start + self.offset, self.range.end + self.offset)
  }

  pub fn apply_middle(&self, range: ValueRange) -> ValueRange {
    ValueRange::new(
      self.range.start + self.offset,
      self.range.end + self.offset,
    )
  }

  pub fn apply_last(&self, range: ValueRange) -> ValueRange {
    ValueRange::new(self.range.start + self.offset, range.end + self.offset)
  }
}

impl Ord for RangeMapping {
  fn cmp(&self, other: &Self) -> Ordering {
    self.range.start.cmp(&other.range.start)
  }
}

impl PartialOrd for RangeMapping {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.range.start.partial_cmp(&other.range.start)
  }
}

impl PartialEq for RangeMapping {
  fn eq(&self, other: &Self) -> bool {
    self.range.start == other.range.start
  }
}

impl Eq for RangeMapping {}
