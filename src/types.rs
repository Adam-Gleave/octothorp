/// NodeLoc structure, representing location within octree
#[derive(Debug)]
pub struct NodeLoc {
    location: Vec<u16>,
}

impl NodeLoc {
    /// Constructs a new `NodeLoc`.
    ///
    /// # Examples
    ///
    /// ```
    /// use octo::types::NodeLoc;
    ///
    /// let loc_a = NodeLoc::new((0, 0, 0));
    ///
    /// let coords = (16, 16, 16);
    /// let loc_b = NodeLoc::new(coords);
    /// ```
    ///
    pub fn new(coords: (u16, u16, u16)) -> NodeLoc {
        NodeLoc {
            location: vec![coords.0, coords.1, coords.2],
        }
    }

    /// Returns the x value of the NodeLoc
    pub fn x(&self) -> u16 {
        self.location[0]
    }

    /// Returns the y value of the NodeLoc
    pub fn y(&self) -> u16 {
        self.location[1]
    }

    /// Returns the z value of the NodeLoc
    pub fn z(&self) -> u16 {
        self.location[2]
    }
}