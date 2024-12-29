pub struct GridInput {
  pub width: usize,
  pub height: usize,
  pub data: Vec<char>,
}

impl GridInput {
  pub fn from_str(input: &str) -> Self {
    let mut width = 0;
    let mut height = 0;
    let mut data = Vec::new();
    for line in input.lines() {
      if width == 0 {
        width = line.len();
      } else if width != line.len() {
        panic!("Non-rectangular input");
      }
      data.extend(line.chars());
      height += 1;
    }

    Self {
      width,
      height,
      data,
    }
  }

  pub fn iter_with_coords(&self) -> XYIterator {
    XYIterator {
      grid: self,
      x: 0,
      y: 0,
    }
  }

  pub fn in_bounds(&self, x: isize, y: isize) -> bool {
    x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
  }

  pub fn at(&self, x: usize, y: usize) -> char {
    self.data[y * self.width + x]
  }
}

pub struct XYIterator<'a> {
  grid: &'a GridInput,
  x: usize,
  y: usize,
}

impl Iterator for XYIterator<'_> {
  type Item = (usize, usize, char);

  fn next(&mut self) -> Option<Self::Item> {
    if self.y >= self.grid.height {
      return None;
    }

    let val = Some((self.x, self.y, self.grid.at(self.x, self.y)));
    if self.x + 1 >= self.grid.width {
      self.x = 0;
      self.y += 1;
    } else {
      self.x += 1;
    }
    val
  }
}
