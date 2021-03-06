use fxhash::FxBuildHasher;
use std::collections::HashSet;

pub type Id = usize;
pub type Capacity = i16;
pub type Time = usize;
pub type TimeDiff = i32;
pub type Fitness = f32;
pub type IdSet = HashSet<Id, BuildHasher>;
pub type BuildHasher = FxBuildHasher;
