/// NodeLoc structure, representing location within octree
pub struct NodeLoc {
    location: Vec<u32>,
}

impl NodeLoc {
    /// Constructs a new `NodeLoc`.
    ///
    /// # Examples
    ///
    /// ```
    /// use types::NodeLoc;
    ///
    /// let loc_a = NodeLoc::new((0, 0, 0));
    ///
    /// let coords = (16, 16, 16);
    /// let loc_b = NodeLoc::new(coords);
    /// ```
    ///
    pub fn new(coords: (u32, u32, u32)) -> NodeLoc {
        NodeLoc {
            location: vec![coords.0, coords.1, coords.2],
        }
    }

    /// Returns the x value of the NodeLoc
    pub fn x(&self) -> u32 {
        self.location[0]
    }

    /// Returns the y value of the NodeLoc
    pub fn y(&self) -> u32 {
        self.location[1]
    }

    /// Returns the z value of the NodeLoc
    pub fn z(&self) -> u32 {
        self.location[2]
    }
}