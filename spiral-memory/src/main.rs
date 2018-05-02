use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;

fn find_ring(num : i64) -> i64 {
    (num as f64).sqrt().ceil() as i64 / 2
}

fn manhattan_distance(num : i64) -> i64 {
    let ring = find_ring(num);
    let ring_length = ring * 2 + 1;
    let diff = (ring_length * ring_length) - num;
    let offset = (diff % (ring * 2) - ring).abs();
    ring + offset
}

#[derive(Eq, Debug, Hash, PartialEq)]
struct Vec2 {
    x : i64,
    y : i64
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vec2 {
    fn rota(&self) -> Vec2 {
        Vec2 {
            x: -self.y,
            y: self.x,
        }
    }
    
    fn rotc(&self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: -self.x,
        }
    }
}

struct Cursor {
    pos : Vec2,
    around : Vec2,
    inward : Vec2,
}

impl Cursor {
    fn new() -> Cursor {
        Cursor {
            pos: Vec2 {x: 0, y: 0},
            around: Vec2 {x: 0, y: -1},
            inward: Vec2 {x: 1, y: 0},
        }
    }
}

fn main() {
    println!("{}", manhattan_distance(277678));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ring() {
        assert_eq!(find_ring(24), 2);
    }

    #[test]
    fn manhattan_distance_works() {
        assert_eq!(manhattan_distance(24), 3);
    }
}