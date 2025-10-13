use std::fmt::Display;

use crate::enums::intercardinals::Intercardinal;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Hexagon {
    pub q: i64,
    pub r: i64,
}

impl Hexagon {
    pub fn origin() -> Self {
        Self { q: 0, r: 0 }
    }

    pub fn s(&self) -> i64 {
        -self.q - self.r
    }   

    pub fn hex_at(&self, dir: Intercardinal) -> Self {
        match dir {
            Intercardinal::North => Self { r: self.r - 1, ..*self },
            Intercardinal::South => Self { r: self.r + 1, ..*self },
            Intercardinal::Northwest => Self { q: self.q - 1, ..*self },
            Intercardinal::Northeast => Self { q: self.q + 1, r: self.r - 1 },
            Intercardinal::Southwest => Self { q: self.q - 1, r: self.r + 1 },
            Intercardinal::Southeast => Self { q: self.q + 1, ..*self },
            _ => { panic!("Hexagons do not have East or West!"); },
        }
    }

    pub fn distance(&self, other: Self) -> usize {
        let vector = Self { q: self.q - other.q, r: self.r - other.r };
        [vector.q.abs(), vector.r.abs(), vector.s().abs()].into_iter()
            .max()
            .unwrap() as usize
    }

    pub fn adjacent(&self) -> [Self; 6] {
        [
            self.hex_at(Intercardinal::Northwest),
            self.hex_at(Intercardinal::North),
            self.hex_at(Intercardinal::Northeast),
            self.hex_at(Intercardinal::Southeast),
            self.hex_at(Intercardinal::South),
            self.hex_at(Intercardinal::Southwest),
        ]
    }
}

impl Display for Hexagon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hex(q={}, r={}, s={})", self.q, self.r, self.s())
    }
}

impl Ord for Hexagon {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance(Self::origin()).cmp(&other.distance(Self::origin()))
    }
}

impl PartialOrd for Hexagon {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
