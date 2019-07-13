use std::error::Error;
use std::fmt;

/// Errors raised by the library
#[derive(Debug)]
pub enum OctreeError {
    DimensionError,
    OutOfBoundsError,
}

impl Error for OctreeError {
    fn description(&self) -> &str {
        match *self {
            OctreeError::DimensionError => {
                "Invalid dimension for octree. Must be an exponent of 2."
            }
            OctreeError::OutOfBoundsError => "Node location provided is out of octree bounds.",
        }
    }
}

impl fmt::Display for OctreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}
