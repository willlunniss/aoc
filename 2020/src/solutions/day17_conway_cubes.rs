use std::fmt;

#[derive(PartialEq, Clone)]
struct Grid {
    len: isize,
    offset: isize,
    points: Vec<Vec<Vec<Vec<State>>>>,
}

impl Grid {
    /// Creates a new Grid
    fn new(size: usize) -> Grid {
        Grid {
            len: size as isize / 2,
            offset: (size + 4) as isize / 2,
            // Init the nested vectors, add +4 so we don't need to worry about boundaries
            points: vec![vec![vec![vec![State::Inactive; size + 4]; size + 4]; size + 4]; size + 4],
        }
    }

    /// Gets the state of a point
    fn get(&self, point: Point) -> State {
        self.points[(point.w + self.offset) as usize][(point.z + self.offset) as usize]
            [(point.y + self.offset) as usize][(point.x + self.offset) as usize]
    }

    /// Set a point to s state
    fn set(&mut self, point: Point, state: State) {
        self.points[(point.w + self.offset) as usize][(point.z + self.offset) as usize]
            [(point.y + self.offset) as usize][(point.x + self.offset) as usize] = state;
    }

    /// Counts the active points
    fn active(&self) -> usize {
        self.points
            .iter()
            .flatten()
            .flatten()
            .flatten()
            .filter(|&x| *x == State::Active)
            .count()
    }

    /// Counts the active neighbours in 3D space
    fn active_neighbours3(&self, point: Point) -> usize {
        let mut count = 0;
        for z in point.z - 1..=point.z + 1 {
            for y in point.y - 1..=point.y + 1 {
                for x in point.x - 1..=point.x + 1 {
                    let neigh = Point::new3(x, y, z);
                    if point != neigh && self.get(neigh) == State::Active {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Counts the active neighbours in 4D space
    fn active_neighbours4(&self, point: Point) -> usize {
        let mut count = 0;
        for w in point.w - 1..=point.w + 1 {
            for z in point.z - 1..=point.z + 1 {
                for y in point.y - 1..=point.y + 1 {
                    for x in point.x - 1..=point.x + 1 {
                        let neigh = Point::new4(x, y, z, w);
                        if point != neigh && self.get(neigh) == State::Active {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    /// Get a list of all points contained within the grid in 3D space
    ///
    /// Can then be used to explore the grid's contents
    pub fn explore3(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for z in -self.len..self.len {
            for y in -self.len..self.len {
                for x in -self.len..self.len {
                    points.push(Point::new3(x, y, z));
                }
            }
        }
        points
    }

    /// Get a list of all points contained within the grid in 4D space
    ///
    /// Can then be used to explore the grid's contents
    pub fn explore4(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for w in -self.len..self.len {
            for z in -self.len..self.len {
                for y in -self.len..self.len {
                    for x in -self.len..self.len {
                        points.push(Point::new4(x, y, z, w));
                    }
                }
            }
        }
        points
    }
}

impl fmt::Debug for Grid {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for z in -self.offset..self.offset {
            // Print all Z planes that have active points (makes it easier to debug)
            // TODO: Add W plane debug
            if self.points[self.offset as usize][(z + self.offset) as usize]
                .iter()
                .flatten()
                .filter(|&x| *x == State::Active)
                .count()
                > 0
            {
                writeln!(f, "z={}", z);
                for y in -self.offset..self.offset {
                    for x in -self.offset..self.offset {
                        write!(f, "{:?}", &self.get(Point::new3(x, y, z)));
                    }
                    write!(f, "\n");
                }
            }
        }
        fmt::Result::Ok(())
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Point {
    w: isize,
    z: isize,
    y: isize,
    x: isize,
}

impl Point {
    /// Create a new Point in 2D space
    fn new2(x: isize, y: isize) -> Point {
        Point {
            w: 0,
            z: 0,
            y: y,
            x: x,
        }
    }

    /// Create a new Point in 3D space
    fn new3(x: isize, y: isize, z: isize) -> Point {
        Point {
            w: 0,
            z: z,
            y: y,
            x: x,
        }
    }

    /// Create a new Point in 4D space
    fn new4(x: isize, y: isize, z: isize, w: isize) -> Point {
        Point {
            w: w,
            z: z,
            y: y,
            x: x,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum State {
    Active,
    Inactive,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Active => write!(f, "#"),
            Self::Inactive => write!(f, "."),
        }
    }
}

impl State {
    /// Create a new State from a char
    fn new(c: char) -> Self {
        match c {
            '#' => Self::Active,
            '.' => Self::Inactive,
            _ => panic!("Unexpected char '{}'", c),
        }
    }

    /// Return the next state based on current state and the supplied
    /// number of active neighbours
    const fn cycle(self, active_neighbours: usize) -> Self {
        match self {
            Self::Active => {
                if active_neighbours == 2 || active_neighbours == 3 {
                    Self::Active
                } else {
                    Self::Inactive
                }
            }
            Self::Inactive => {
                if active_neighbours == 3 {
                    Self::Active
                } else {
                    Self::Inactive
                }
            }
        }
    }
}

#[aoc_generator(day17)]
fn gen(input: &str) -> Grid {
    // Assumption: The we get an NxN slice to start
    // Assumption: We won't simulate for more than 6 cycles and we won't grow
    // by more than 1 per cycle in EACH direction so can set overall size to N + 12
    let start_size = input.lines().next().unwrap().len();
    let offset = start_size as isize / 2;
    let mut grid = Grid::new(start_size + 12);
    for (y, line) in input.lines().enumerate() {
        let y_centred = y as isize - offset;
        for (x, state) in line.chars().enumerate() {
            let x_centred = x as isize - offset;
            grid.set(Point::new2(x_centred, y_centred), State::new(state));
        }
    }

    grid
}

#[aoc(day17, part1)]
fn part1(input: &Grid) -> usize {
    let mut current = input.clone();
    let mut next = input.clone();
    for _cycle in 1..=6 {
        std::mem::swap(&mut next, &mut current);
        for point in current.explore3() {
            let count = current.active_neighbours3(point);
            next.set(point, current.get(point).cycle(count));
        }
    }
    next.active()
}

#[aoc(day17, part2)]
fn part2(input: &Grid) -> usize {
    let mut current = input.clone();
    let mut next = input.clone();
    for _cycle in 1..=6 {
        std::mem::swap(&mut next, &mut current);
        for point in current.explore4() {
            let count = current.active_neighbours4(point);
            next.set(point, current.get(point).cycle(count));
        }
    }
    next.active()
}
