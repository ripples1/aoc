use core::panic;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
  x: isize,
  y: isize,
}

impl Coord {
  const fn new(x: isize, y: isize) -> Self {
    Coord { x, y }
  }

  const fn normal(&self) -> Self {
    Coord::new(-self.y, self.x)
  }

  const ZERO: Self = Coord::new(0, 0);
}

impl std::ops::Add for Coord {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Coord::new(self.x + other.x, self.y + other.y)
  }
}

impl std::ops::Sub for Coord {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Coord::new(self.x - other.x, self.y - other.y)
  }
}

impl std::ops::Neg for Coord {
  type Output = Self;

  fn neg(self) -> Self {
    Coord::new(-self.x, -self.y)
  }
}

impl Ord for Coord {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.x.cmp(&other.x).then(self.y.cmp(&other.y))
  }
}

impl PartialOrd for Coord {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct DirMarker {
  in_dir: Coord,
  out_dir: Coord,
}

impl DirMarker {
  const fn new(in_dir: Coord, out_dir: Coord) -> Self {
    DirMarker { in_dir, out_dir }
  }

  const ZERO: Self = DirMarker::new(Coord::ZERO, Coord::ZERO);
}

#[derive(Debug)]
struct Field {
  width: isize,
  height: isize,
  pipes: Vec<char>,
  loop_markers: Vec<char>,
  dir_markers: Vec<DirMarker>,
  convex_f_point: Coord,
}

const CONNECTORS_NORTH: [char; 3] = ['7', '|', 'F'];
const CONNECTORS_EAST: [char; 3] = ['J', '-', '7'];
const CONNECTORS_SOUTH: [char; 3] = ['L', '|', 'J'];
const CONNECTORS_WEST: [char; 3] = ['F', '-', 'L'];

impl Field {
  fn new<'a>(data: impl Iterator<Item = &'a str>) -> Self {
    let mut chars = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in data {
      if width == 0 {
        width = line.len();
      } else if width != line.len() {
        panic!("Non-rectangular field");
      }
      chars.extend(line.chars());
      height += 1;
    }

    Field {
      width: width as isize,
      height: height as isize,
      pipes: chars,
      loop_markers: vec!['.'; width * height],
      dir_markers: vec![DirMarker::ZERO; width * height],
      convex_f_point: Coord::new((width - 1) as isize, (height - 1) as isize),
    }
  }

  fn print(&self) {
    for y in 0..self.height {
      for x in 0..self.width {
        print!("{}", self.get(Coord::new(x, y)).unwrap());
      }
      println!();
    }
    println!();
    for y in 0..self.height {
      for x in 0..self.width {
        if Coord::new(x, y) == self.convex_f_point {
          print!("@");
        } else {
          print!("{}", self.get_marker(Coord::new(x, y)).unwrap());
        }
      }
      println!();
    }
    println!();
    for y in 0..self.height {
      for x in 0..self.width {
        print!("{}", match self.get_dir(Coord::new(x, y)).unwrap().out_dir {
          Coord { x: 0, y: -1 } => '^',
          Coord { x: 1, y: 0 } => '>',
          Coord { x: 0, y: 1 } => 'v',
          Coord { x: -1, y: 0 } => '<',
          _ => '.',
        });
      }
      println!();
    }
  }

  // returns the half-length of the loop
  fn traverse(&mut self) -> i64 {
    let mut current = self.find('S');
    let mut steps = 0;
    loop {
      if current < self.convex_f_point {
        self.convex_f_point = current;
      }
      let (n0, n1) = self.neighbours(current);
      self.set_marker(current, '*');
      steps += 1;
      if self.get_marker(n0).is_some_and(|v| v != '*') {
        current = n0;
      } else if self.get_marker(n1).is_some_and(|v| v != '*') {
        current = n1;
      } else {
        break;
      }
    }
    steps / 2
  }

  /*
 Default direction - counter-clockwise
  F - 7
  |   |
  L - J
 */

  fn compute_dir_markers(&mut self) {
    self.set_dir(
      self.convex_f_point,
      DirMarker::new(Coord::new(1, 0), Coord::new(0, 1))
    );
    let mut prev_coord = self.convex_f_point;
    let mut current_coord = self.convex_f_point + Coord::new(0, 1);
    loop {
      let (n0, n1) = self.neighbours(current_coord);
      let next_coord = if n0 == prev_coord { n1 } else { n0 };
      self.set_dir(
        current_coord,
        DirMarker::new(prev_coord - current_coord, next_coord - current_coord)
      );
      if next_coord == self.convex_f_point || self.get(next_coord).is_none() {
        break;
      }
      prev_coord = current_coord;
      current_coord = next_coord;
    }
  }

  fn area(&mut self) -> i64 {
    let mut area_inside = 0;
    for y in 0..self.height {
      for x in 0..self.width {
        let c = Coord::new(x, y);
        if self.is_main_loop(c) {
          continue;
        }

        let inside = self.is_inside(c);
        if inside {
          area_inside += 1;
        }
        self.set_marker(c, if inside { 'I' } else { 'O' });
      }
    }
    area_inside
  }

  fn is_inside(&self, coord: Coord) -> bool {
    let mut current = coord;
    let dir = self.direction_to_the_closest_edge(coord);
    loop {
      let next = current + dir;
      if self.get(next).is_none() {
        return false;
      }
      if self.is_main_loop(next) {
        let dir = self.get_dir(next).unwrap();
        return !(
          dir.out_dir.normal() + next == current ||
          -dir.in_dir.normal() + next == current
        );
      }
      current = next;
    }
  }

  fn direction_to_the_closest_edge(&self, coord: Coord) -> Coord {
    let dir_x = if coord.x < self.width / 2 { -1 } else { 1 };
    let dir_y = if coord.y < self.height / 2 { -1 } else { 1 };
    let dist_x = if dir_x == -1 { coord.x } else { self.width - coord.x };
    let dist_y = if dir_y == -1 { coord.y } else { self.height - coord.y };
    if dist_x < dist_y {
      Coord::new(dir_x, 0)
    } else {
      Coord::new(0, dir_y)
    }
  }

  fn find(&self, what: char) -> Coord {
    for y in 0..self.height {
      for x in 0..self.width {
        let c = Coord::new(x, y);
        if self.get(c).is_some_and(|v| v == what) {
          return c;
        }
      }
    }
    panic!("Not found");
  }

  fn neighbours(&self, coord: Coord) -> (Coord, Coord) {
    match self.get(coord).expect("Invalid coord") {
      '|' =>
        (Coord::new(coord.x, coord.y - 1), Coord::new(coord.x, coord.y + 1)),
      '-' =>
        (Coord::new(coord.x - 1, coord.y), Coord::new(coord.x + 1, coord.y)),
      'F' =>
        (Coord::new(coord.x, coord.y + 1), Coord::new(coord.x + 1, coord.y)),
      '7' =>
        (Coord::new(coord.x - 1, coord.y), Coord::new(coord.x, coord.y + 1)),
      'J' =>
        (Coord::new(coord.x, coord.y - 1), Coord::new(coord.x - 1, coord.y)),
      'L' =>
        (Coord::new(coord.x, coord.y - 1), Coord::new(coord.x + 1, coord.y)),
      'S' => {
        let mut res = Vec::with_capacity(2);

        let coord_north = Coord::new(coord.x, coord.y - 1);
        let coord_east = Coord::new(coord.x + 1, coord.y);
        let coord_south = Coord::new(coord.x, coord.y + 1);
        let coord_west = Coord::new(coord.x - 1, coord.y);

        if self.get(coord_north).is_some_and(|v| CONNECTORS_NORTH.contains(&v)) {
          res.push(coord_north);
        }
        if self.get(coord_east).is_some_and(|v| CONNECTORS_EAST.contains(&v)) {
          res.push(coord_east);
        }
        if self.get(coord_south).is_some_and(|v| CONNECTORS_SOUTH.contains(&v)) {
          res.push(coord_south);
        }
        if self.get(coord_west).is_some_and(|v| CONNECTORS_WEST.contains(&v)) {
          res.push(coord_west);
        }

        (res[0], res[1])
      }
      c @ _ => panic!("Invalid direction: {}", c),
    }
  }

  fn bound_check(&self, coord: Coord) -> bool {
    coord.x >= 0 &&
      coord.x < self.width &&
      coord.y >= 0 &&
      coord.y < self.height
  }

  fn linear_addr(&self, coord: Coord) -> usize {
    (coord.y * self.width + coord.x) as usize
  }

  fn get(&self, coord: Coord) -> Option<char> {
    if !self.bound_check(coord) {
      return None;
    }
    Some(self.pipes[self.linear_addr(coord)])
  }

  fn set(&mut self, coord: Coord, value: char) {
    if !self.bound_check(coord) {
      return;
    }
    let addr = self.linear_addr(coord);
    self.pipes[addr] = value;
  }

  fn is_main_loop(&self, coord: Coord) -> bool {
    self.get_marker(coord).is_some_and(|v| v == '*')
  }

  fn get_marker(&self, coord: Coord) -> Option<char> {
    if !self.bound_check(coord) {
      return None;
    }
    Some(self.loop_markers[self.linear_addr(coord)])
  }

  fn set_marker(&mut self, coord: Coord, value: char) {
    if !self.bound_check(coord) {
      return;
    }
    let addr = self.linear_addr(coord);
    self.loop_markers[addr] = value;
  }

  fn get_dir(&self, coord: Coord) -> Option<DirMarker> {
    if !self.bound_check(coord) {
      return None;
    }
    Some(self.dir_markers[self.linear_addr(coord)])
  }

  fn set_dir(&mut self, coord: Coord, value: DirMarker) {
    if !self.bound_check(coord) {
      return;
    }
    let addr = self.linear_addr(coord);
    self.dir_markers[addr] = value;
  }
}

pub fn part_1(input: &str) -> i64 {
  let mut fld = Field::new(input.lines());
  let v = fld.traverse();
  fld.print();
  v
}

pub fn part_2(input: &str) -> i64 {
  let mut fld = Field::new(input.lines());
  fld.traverse();
  fld.compute_dir_markers();
  let a = fld.area();
  fld.print();
  a
}
