use std::collections::HashMap;
use crate::nbt::nbt_type::NbtType;
use std::io;
use core::fmt;


#[derive(Clone, Debug, PartialEq)]
pub struct NbtTag {
    label: String,
    tag_type: HashMap<String, NbtType>,
}

impl NbtTag {
    pub fn new() -> NbtTag {
        NbtTag {
            label: "".to_string(),
            tag_type: HashMap::new(),
        }
    }

    pub fn with_name<N>(name: N) -> NbtTag where N: Into<String> {
        NbtTag {
            label: name.into(),
            tag_type: HashMap::new(),
        }
    }

    pub fn get<N>(&self, name: N) -> Option<&NbtType> where N: Into<&'static str> {
        self.tag_type.get(name.into())
    }
}

impl fmt::Display for NbtTag {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "TAG_Compound(\"{}\"): {} entry(ies)\n{{\n",
            self.title,
            self.content.len()
        )?;

        for (name, tag) in self.content.iter() {
            write!(formatter, "  {}(\"{}\"): ", tag.tag_name(), name)?;
            tag.print(formatter, 2)?;
            writeln!(formatter)?;
        }

        write!(formatter, "}}")
    }
}