//!
//!  point.rs
//!
//!  Created by Mitchell Nordine at 07:23PM on July 27, 2014.
//!
//!

use std::default::Default;

pub struct Point<T> {
    x: T,
    y: T,
    z: T,
}

impl<T: Num + Copy> Point<T> {

    /// Constructor for a point.
    pub fn new(x: T, y: T, z: T) -> Point<T> {
        Point{ x: x, y: y, z: z }
    }

    /// Return the point as a vector.
    pub fn as_vec(&self) -> Vec<T> {
        vec![self.x, self.y, self.z]
    }

    /// Return the point as a tuple.
    pub fn as_tuple(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

}

impl<T: Num + Copy + Default> Default for Point<T> {
    fn default() -> Point<T> {
        Point::new(Default::default(), Default::default(), Default::default())
    }
}
