use std::ops::Range;

/// Return true if two range overlap.
/// 
/// assert_eq!(ranges::overlap(0..7,3..10),true)

pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool{
    r1.start < r1.end && r2.start < r2.end &&
      r1.start < r2.end && r2.start < r1.end
}