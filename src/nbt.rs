use crate::binary::{FileReader, FileWriter, TagIo, Writer, BinResult};
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
    pub fn from_be_bytes(bytes: &Vec<u8>) -> BinResult<Self> {
        let mut fr = FileReader::new(bytes, 0);
        //set initial tag to compound as all data is implicitly in a compound
        Self::read_be(0x0A, &mut fr)
    }

    pub fn from_le_bytes(bytes: &Vec<u8>) -> BinResult<Self> {
        let mut fr = FileReader::new(bytes, 0);
        //set initial tag to compound as all data is implicitly in a compound
        Self::read_le(0x0A, &mut fr)
    }

    pub fn to_be_bytes(v: Self) -> Vec<u8> {
        let mut fw = FileWriter::new();
        fw.write_be(&v);
        fw.bytes()
    }

    pub fn to_le_bytes(v: Self) -> Vec<u8> {
        let mut fw = FileWriter::new();
        fw.write_be(&v);
        fw.bytes()
    }

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
    pub fn wrapped(self, k: String) -> Tag {
        let mut buf = HashMap::<String, Tag>::new();
        buf.insert(k, self);
        Tag::Compound(buf)
    }
}

macro_rules! read_array_tag {
    ($type:ident, $fr:expr, $endian:ident) => {
        {
            let len: i32 = $fr.$endian()?;
            let mut array = Vec::new();
            for _ in 0..len {
                array.push($fr.$endian()?);
            }
            Ok(Tag::$type(array))
        }
    };
}

macro_rules! read_endian {
    ($endian:ident) => {
        fn $endian(tag_id: u8, fr: &mut FileReader) -> BinResult<Self> {
            match tag_id {
                0x01 => Ok(Tag::Byte(fr.$endian()?)),
                0x02 => Ok(Tag::Short(fr.$endian()?)),
                0x03 => Ok(Tag::Int(fr.$endian()?)),
                0x04 => Ok(Tag::Long(fr.$endian()?)),
                0x05 => Ok(Tag::Float(fr.$endian()?)),
                0x06 => Ok(Tag::Double(fr.$endian()?)),
                0x07 => read_array_tag!(ByteArray, fr, $endian),
                0x08 => Ok(Tag::String(fr.$endian()?)),
                0x09 => {
                    let type_id: u8 = fr.$endian()?;
                    let len: i32 = fr.$endian()?;
                    let mut array = Vec::<Tag>::new();
                    for _ in 0..len {
                        array.push(Tag::$endian(type_id, fr)?);
                    }
                    Ok(Tag::List(array))
                }
                0x0A => {
                    let mut buf = HashMap::<String, Tag>::new();
                    while !fr.at_end() {
                        let tag_id: u8 = fr.$endian()?;
                        if tag_id == 0x00 {
                            break;
                        }
                        buf.insert(fr.$endian()?, Tag::$endian(tag_id, fr)?);
                    }
                    Ok(Tag::Compound(buf))
                }
                0x0B => read_array_tag!(IntArray, fr, $endian),
                0x0C => read_array_tag!(LongArray, fr, $endian),
                x => {
                    panic!("Invalid Tag ID: {}", x)
                }
            }
        }
    };
}

impl TagIo for Tag {
    read_endian!(read_be);
    read_endian!(read_le);
}

macro_rules! write_array_tag {
    ($v:expr, $fw:expr, $endian:ident) => {
        {
            $fw.$endian(&($v.len() as i32));
            for i in 0..$v.len() {
                $fw.$endian(&$v[i]);
            }
        }
    };
}

macro_rules! write_endian {
    ($endian:ident) => {
        fn $endian(&self, fw: &mut FileWriter) {
            match self {
                Tag::Byte(v) => fw.$endian(v),
                Tag::Short(v) => fw.$endian(v),
                Tag::Int(v) => fw.$endian(v),
                Tag::Long(v) => fw.$endian(v),
                Tag::Float(v) => fw.$endian(v),
                Tag::Double(v) => fw.$endian(v),
                Tag::ByteArray(v) => write_array_tag!(v, fw, $endian),
                Tag::String(v) => fw.$endian(v),
                Tag::List(v) => {
                    if v.len() == 0 {
                        fw.$endian::<u8>(&0x00);
                    } else {
                        fw.$endian(&v[0].tag_id());
                    }
                    fw.$endian::<i32>(&(v.len() as i32));
                    for i in 0..v.len() {
                        v[i].$endian(fw);
                    }
                }
                Tag::Compound(map) => {
                    for (k, v) in map.iter() {
                        fw.$endian(&v.tag_id());
                        fw.$endian(k);
                        fw.$endian(v);
                    }
                    fw.$endian::<u8>(&0x00);
                }
                Tag::IntArray(v) => write_array_tag!(v, fw, $endian),
                Tag::LongArray(v) => write_array_tag!(v, fw, $endian),
            }
        }
    };
}

impl Writer for Tag {
    write_endian!(write_be);
    write_endian!(write_le);
}