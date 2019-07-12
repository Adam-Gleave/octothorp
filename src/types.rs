/// NodeLoc structure, representing location within octree
#[derive(Debug)]
pub struct NodeLoc {
    location: Vec<u16>,
}

impl NodeLoc {
    /// Constructs a new `NodeLoc`.
    pub fn new(coords: (u16, u16, u16)) -> NodeLoc {
        NodeLoc {
            location: vec![coords.0, coords.1, coords.2],
        }
    }

    /// Returns the x value of the `NodeLoc`
    pub fn x(&self) -> u16 {
        self.location[0]
    }

    /// Returns the y value of the `NodeLoc`
    pub fn y(&self) -> u16 {
        self.location[1]
    }

    /// Returns the z value of the `NodeLoc`
    pub fn z(&self) -> u16 {
        self.location[2]
    }

    /// Set the `NodeLoc` x value
    pub fn sub_x(&mut self, delta: u16) {
        self.location[0 as usize] -= delta;
    }

    /// Set the `NodeLoc` y value
    pub fn sub_y(&mut self, delta: u16) {
        self.location[1 as usize] -= delta;
    }

    /// Set the `NodeLoc` z value
    pub fn sub_z(&mut self, delta: u16) {
        self.location[2 as usize] -= delta;
    }
}
