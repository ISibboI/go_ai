use tritvector::{TritVector, Trit};
use bitvector::BitVector;
use std::collections::VecDeque;

pub mod tritvector;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GoStone {
    NONE, BLACK, WHITE
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GoCoordinates {
    x: u8,
    y: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoBoard {
    board: TritVector,
}

#[derive(Clone, Debug)]
pub struct GoBoardMask {
    mask: BitVector,
}

impl GoBoardMask {
    pub fn new() -> Self {
        Self {mask: BitVector::new(9 * 9)}
    }

    pub fn get(&self, c: GoCoordinates) -> bool {
        self.mask.contains(c.into())
    }

    pub fn set(&mut self, c: GoCoordinates, value: bool) {
        if value {
            self.mask.insert(c.into());
        } else {
            self.mask.remove(c.into());
        }
    }
}

impl GoBoard {
    pub fn new() -> Self {
        Self {board: TritVector::new(9 * 9)}
    }

    pub fn get_stone(&self, coordinates: GoCoordinates) -> GoStone {
        self.board.get(coordinates.into()).into()
    }

    pub fn set_stone(&mut self, coordinates: GoCoordinates, stone: GoStone) {
        self.board.set(coordinates.into(), stone.into());
    }

    pub fn kill_stones(&mut self, coordinates: GoCoordinates) -> u64 {
        let killer = self.get_stone(coordinates);
        let killed = killer.opponent_color();
        let mut result = 0;
        for neighbor in coordinates.neighbors() {
            let neighbor_color = self.get_stone(neighbor);

            if neighbor_color == killed && !self.group_has_liberties(neighbor) {
                result += self.remove_group(neighbor);
            }
        }

        result
    }

    pub fn group_has_liberties(&self, coordinates: GoCoordinates) -> bool {
        let color = self.get_stone(coordinates);
        assert!(!color.is_none());

        let mut enqueued = GoBoardMask::new();
        enqueued.set(coordinates, true);

        let mut queue = VecDeque::new();
        queue.push_back(coordinates);

        while let Some(coordinates) = queue.pop_back() {
            assert_eq!(self.get_stone(coordinates), color);

            for neighbor in coordinates.neighbors() {
                let neighbor_color = self.get_stone(neighbor);
                if neighbor_color.is_none() {
                    return true;
                }

                if neighbor_color == color && !enqueued.get(neighbor) {
                    enqueued.set(neighbor, true);
                    queue.push_back(neighbor);
                }
            }
        }

        return false;
    }

    pub fn remove_group(&mut self, coordinates: GoCoordinates) -> u64 {
        let color = self.get_stone(coordinates);
        assert!(!color.is_none());

        let mut queue = VecDeque::new();
        queue.push_back(coordinates);
        self.set_stone(coordinates, GoStone::NONE);
        let mut result = 1;

        while let Some(coordinates) = queue.pop_back() {
            assert!(self.get_stone(coordinates).is_none());

            for neighbor in coordinates.neighbors() {
                let neighbor_color = self.get_stone(neighbor);

                if neighbor_color == color {
                    self.set_stone(neighbor, GoStone::NONE);
                    result += 1;
                    queue.push_back(neighbor);
                }
            }
        }

        result
    }
}


impl GoCoordinates {
    pub fn new(x: u8, y: u8) -> Self {
        assert!(x < 9);
        assert!(y < 9);
        Self {x, y}
    }

    pub fn new_usize(x: usize, y: usize) -> Self {
        assert!(x < 9);
        assert!(y < 9);
        Self {x: x as u8, y: y as u8}
    }

    pub fn neighbors(&self) -> Vec<GoCoordinates> {
        let mut result = Vec::new();
        if self.x > 0 {
            result.push(GoCoordinates {x: self.x - 1, y: self.y});
        }
        if self.y > 0 {
            result.push(GoCoordinates {x: self.x, y: self.y - 1});
        }
        if self.x < 8 {
            result.push(GoCoordinates {x: self.x + 1, y: self.y});
        }
        if self.y < 8 {
            result.push(GoCoordinates {x: self.x, y: self.y + 1});
        }
        result
    }
}

impl GoStone {
    pub fn opponent_color(&self) -> Self {
        match self {
            GoStone::BLACK => GoStone::WHITE,
            GoStone::WHITE => GoStone::BLACK,
            GoStone::NONE => panic!("Not a stone")
        }
    }

    pub fn is_none(&self) -> bool {
        *self == GoStone::NONE
    }
}

impl From<usize> for GoCoordinates {
    fn from(i: usize) -> Self {
        assert!(i < 81);
        let i = i as u8;
        Self {x: i % 9, y: i / 9}
    }
}

impl From<GoCoordinates> for usize {
    fn from(go_coordinates: GoCoordinates) -> Self {
        assert!(go_coordinates.x < 9);
        assert!(go_coordinates.y < 9);

        (go_coordinates.x + 9 * go_coordinates.y).into()
    }
}

impl From<GoStone> for Trit {
    fn from(go_stone: GoStone) -> Self {
        match go_stone {
            GoStone::NONE => Trit::ZERO,
            GoStone::BLACK => Trit::ONE,
            GoStone::WHITE => Trit::TWO,
        }
    }
}

impl From<Trit> for GoStone {
    fn from(trit: Trit) -> Self {
        match trit {
            Trit::ZERO => GoStone::NONE,
            Trit::ONE => GoStone::BLACK,
            Trit::TWO => GoStone::WHITE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GoCoordinates;

    #[test]
    fn test_coordinate_transformation() {
        for i in 0..81 {
            assert_eq!(i, usize::from(GoCoordinates::from(i)));
        }
    }
}