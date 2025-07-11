use std::hash::Hash;

use serde::{Deserialize, Serialize};

pub mod nodes;
pub mod treebuilder;
/// All possible Datatypes
#[derive(Clone, Serialize, Deserialize)]
pub enum Data {
    Folder,
    Button(u64), // how often pressed
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

impl Hash for Data {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Folder => state.write_u8(0),
            Self::Button(n) => {
                state.write_u8(1);
                state.write_u64(*n);
            }
            Self::Float32(v) => {
                state.write_u8(2);
                state.write_u32(v.to_bits());
            }
            Self::Float64(v) => {
                state.write_u8(3);
                state.write_u64(v.to_bits());
            }
            Self::Int32(v) => state.write_i32(*v),
            Self::Int64(v) => state.write_i64(*v),
            Self::UInt32(v) => state.write_u32(*v),
            Self::UInt64(v) => state.write_u64(*v),
            Self::String(s) => state.write(s.as_bytes()),
            Self::Bool(v) => state.write_u8(if *v { 1 } else { 0 }),
            Self::Tuple(_, data) => {
                for d in data {
                    d.hash(state);
                }
            }
            Self::List(ls) => {
                for d in ls.iter() {
                    d.hash(state);
                }
            }
        }
    }
}
