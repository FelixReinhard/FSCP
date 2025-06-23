pub mod nodes;
/// All possible Datatypes
pub enum Data {
    Folder,
    Float32(f32),
    Float64(f64),
    Int32(i32),
    Int64(i64),
    UInt32(u32),
    UInt64(u64),
    String(String),
    Bool(bool),
    Tuple(usize, Box<[Data]>),
    List(Box<Vec<Data>>),
}
