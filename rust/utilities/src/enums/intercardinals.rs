use super::cardinals::Cardinal;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Intercardinal {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl From<&str> for Intercardinal {
    fn from(direction: &str) -> Self {
        Self::new(direction).unwrap()
    }
}

impl Intercardinal {
    pub fn new(direction: &str) -> Option<Self> {
        match direction.to_ascii_uppercase().as_str() {
            "NW" => Some(Self::Northwest),
            "N" => Some(Self::North),
            "NE" => Some(Self::Northeast),
            "E" => Some(Self::East),
            "SE" => Some(Self::Southeast),
            "S" => Some(Self::South),
            "SW" => Some(Self::Southwest),
            "W" => Some(Self::West),
            _ => None,
        }
    }

    pub fn ordinal(&self) -> usize {
        match self {
            Self::Northwest => 0,
            Self::North => 1,
            Self::Northeast => 2,
            Self::East => 3,
            Self::Southeast => 4,
            Self::South => 5,
            Self::Southwest => 6,
            Self::West => 7,
        }
    }

    pub fn entries() -> [Self; 8] {
        [
            Self::Northwest,
            Self::North,
            Self::Northeast,
            Self::East,
            Self::Southeast,
            Self::South,
            Self::Southwest,
            Self::West,
        ]
    }

    pub fn cardinal_entries() -> [Self; 4] {
        [Self::North, Self::East, Self::South, Self::West]
    }

    pub fn left(&self) -> Self {
        Self::entries()[(self.ordinal() as isize - 2).rem_euclid(8) as usize]
    }

    pub fn right(&self) -> Self {
        Self::entries()[(self.ordinal() + 2) % 8]
    }

    pub fn straight(&self) -> Self {
        *self
    }

    pub fn flip(&self) -> Self {
        Self::entries()[(self.ordinal() + 4) % 8]
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

    pub fn to_cardinal(&self) -> Option<Cardinal> {
        match self {
            Intercardinal::North => Some(Cardinal::North),
            Intercardinal::East => Some(Cardinal::East),
            Intercardinal::South => Some(Cardinal::South),
            Intercardinal::West => Some(Cardinal::West),
            _ => None,
        }
    }
}
