#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Cardinal {
    pub fn new(direction: char) -> Option<Self> {
        match direction.to_ascii_uppercase() {
            'N' => Some(Self::North),
            'E' => Some(Self::East),
            'S' => Some(Self::South),
            'W' => Some(Self::West),
            _ => None,
        }
    }

    pub fn ordinal(&self) -> usize {
        match self {
            Cardinal::North => 0,
            Cardinal::East => 1,
            Cardinal::South => 2,
            Cardinal::West => 3,
        }
    }

    pub fn entries() -> [Self; 4] {
        [Self::North, Self::East, Self::South, Self::West]
    }

    pub fn left(&self) -> Self {
        Self::entries()[(self.ordinal() as isize - 1).rem_euclid(4) as usize]
    }

    pub fn right(&self) -> Self {
        Self::entries()[(self.ordinal() + 1) % 4]
    }

    pub fn straight(&self) -> Self {
        *self
    }

    pub fn flip(&self) -> Self {
        Self::entries()[(self.ordinal() + 2) % 4]
    }

    pub fn turn(&self, direction: char) -> Option<Self> {
        match direction.to_ascii_uppercase() {
            'L' => Some(self.left()),
            'R' => Some(self.right()),
            'F' => Some(self.straight()),
            'B' => Some(self.flip()),
            _ => None,
        }
    }
}