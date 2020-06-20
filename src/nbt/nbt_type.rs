use std::collections::HashMap;
use std::fmt;
use std::io;

use byteorder::{ByteOrder, BigEndian, WriteBytesExt, ReadBytesExt};

use error::{Error, Result};

pub enum NbtType {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    List(Vec<NbtType>),
    Compound(HashMap<String, NbtType>),
    ByteArray(Vec<i8>),
    IntArray(Vec<i32>),
}

impl NbtType {
    pub fn id(&self) -> u8 {
        match *self {
            NbtType::Byte(_)      => 0x01,
            NbtType::Short(_)     => 0x02,
            NbtType::Int(_)       => 0x03,
            NbtType::Long(_)      => 0x04,
            NbtType::Float(_)     => 0x05,
            NbtType::Double(_)    => 0x06,
            NbtType::ByteArray(_) => 0x07,
            NbtType::String(_)    => 0x08,
            NbtType::List(_)      => 0x09,
            NbtType::Compound(_)  => 0x0a,
            NbtType::IntArray(_)  => 0x0b
        }
    }

    fn name(&self) -> &str {
        match *self {
            NbtType::Byte(_)      => "TAG_Byte",
            NbtType::Short(_)     => "TAG_Short",
            NbtType::Int(_)       => "TAG_Int",
            NbtType::Long(_)      => "TAG_Long",
            NbtType::Float(_)     => "TAG_Float",
            NbtType::Double(_)    => "TAG_Double",
            NbtType::ByteArray(_) => "TAG_ByteArray",
            NbtType::String(_)    => "TAG_String",
            NbtType::List(_)      => "TAG_List",
            NbtType::Compound(_)  => "TAG_Compound",
            NbtType::IntArray(_)  => "TAG_IntArray"
        }
    }
}