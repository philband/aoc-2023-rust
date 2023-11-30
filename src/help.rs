#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point(i32, i32);

use itertools::Itertools;

impl From<&(i32, i32)>for Point {
    fn from(data: &(i32, i32)) -> Point {
        Point(data.0, data.1)
    }
}

impl Point {
    pub fn neighbors(&self) -> Vec<Point> {
        (self.0-1..=self.0+1)
            .cartesian_product(self.1-1..=self.1+1)
            .filter(|other| self.manhattan(&Point(other.0, other.1)) != 0)
            .map(|(x, y)| Point(x,y))
            .collect()
    }

    pub fn neighbors_4way(&self) -> Vec<Point> {
        self
            .neighbors()
            .into_iter()
            .filter(|other| self.manhattan(other) == 1)
            .collect()
    }

    pub fn manhattan(&self, other: &Self) -> i32 {
        (self.0-other.0).abs() + (self.1-other.1).abs()
    }
}