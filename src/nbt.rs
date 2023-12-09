use crate::binary::{
    FileReader,
    FileWriter,
    TagIo,
    Writer,
    BinResult
};
use std::{fmt::Debug, collections::HashMap};

#[repr(u8)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Tag {
    Byte(i8) = 1,
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag {
    fn tag_id(&self) -> u8 {
        match self {
            Tag::Byte(_) => {0x01}
            Tag::Short(_) => {0x02}
            Tag::Int(_) => {0x03}
            Tag::Long(_) => {0x04}
            Tag::Float(_) => {0x05}
            Tag::Double(_) => {0x06}
            Tag::ByteArray(_) => {0x07}
            Tag::String(_) => {0x08}
            Tag::List(_) => {0x09}
            Tag::Compound(_) => {0x0A}
            Tag::IntArray(_) => {0x0B}
            Tag::LongArray(_) => {0x0C}
        }
    }

    ///wraps the tag in a compound with it's key/name set to `k`
    pub fn wrapped(self, k: String) -> Self {
        let mut buf = HashMap::<String, Tag>::new();
        buf.insert(k, self);
        Tag::Compound(buf)
    }
}

macro_rules! read_array_tag {
    ($type:ident, $fr:expr) => {
        {
            let len: i32 = $fr.read()?;
            let mut array = Vec::new();
            for _ in 0..len {
                array.push($fr.read()?);
            }
            Ok(Tag::$type(array))
        }
    };
}

macro_rules! write_array_tag {
    ($v:expr, $fw:expr) => {
        {
            $fw.write(&($v.len() as i32));
            for i in 0..$v.len() {
                $fw.write(&$v[i]);
            }
        }
    };
}

impl TagIo for Tag {
    fn read(tag_id: u8, fr: &mut impl FileReader) -> BinResult<Self> {
        match tag_id {
            0x01 => Ok(Tag::Byte(fr.read()?)),
            0x02 => Ok(Tag::Short(fr.read()?)),
            0x03 => Ok(Tag::Int(fr.read()?)),
            0x04 => Ok(Tag::Long(fr.read()?)),
            0x05 => Ok(Tag::Float(fr.read()?)),
            0x06 => Ok(Tag::Double(fr.read()?)),
            0x07 => read_array_tag!(ByteArray, fr),
            0x08 => Ok(Tag::String(fr.read()?)),
            0x09 => {
                let type_id: u8 = fr.read()?;
                let len: i32 = fr.read()?;
                let mut array = Vec::<Tag>::new();
                for _ in 0..len {
                    array.push(Tag::read(type_id, fr)?);
                }
                Ok(Tag::List(array))
            }
            0x0A => {
                let mut buf = HashMap::<String, Tag>::new();
                while !fr.at_end() {
                    let tag_id: u8 = fr.read()?;
                    if tag_id == 0x00 {
                        break;
                    }
                    buf.insert(fr.read()?, Tag::read(tag_id, fr)?);
                }
                Ok(Tag::Compound(buf))
            }
            0x0B => read_array_tag!(IntArray, fr),
            0x0C => read_array_tag!(LongArray, fr),
            x => {
                panic!("Invalid Tag ID: {}", x)
            }
        }
    }
}

impl Writer for Tag {
    fn write(&self, fw: &mut impl FileWriter) {
        match self {
            Tag::Byte(v) => fw.write(v),
            Tag::Short(v) => fw.write(v),
            Tag::Int(v) => fw.write(v),
            Tag::Long(v) => fw.write(v),
            Tag::Float(v) => fw.write(v),
            Tag::Double(v) => fw.write(v),
            Tag::ByteArray(v) => write_array_tag!(v, fw),
            Tag::String(v) => fw.write(v),
            Tag::List(v) => {
                if v.len() == 0 {
                    fw.write::<u8>(&0x00);
                } else {
                    fw.write(&v[0].tag_id());
                }
                fw.write::<i32>(&(v.len() as i32));
                for i in 0..v.len() {
                    v[i].write(fw);
                }
            }
            Tag::Compound(map) => {
                for (k, v) in map.iter() {
                    fw.write(&v.tag_id());
                    fw.write(k);
                    fw.write(v);
                }
                fw.write::<u8>(&0x00);
            }
            Tag::IntArray(v) => write_array_tag!(v, fw),
            Tag::LongArray(v) => write_array_tag!(v, fw),
        }
    }
}