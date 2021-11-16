pub type Id = usize;
pub type Capacity = i32;
pub type Time = usize;

#[derive(Clone, PartialEq)]
pub enum OptionalId {
    AnI32(Id),
    Nothing,
}
