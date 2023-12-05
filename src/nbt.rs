use crate::bin::{FileParser, FileWriter, TagIo};
use std::{fmt::Debug, collections::HashMap};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Tag {
    End,
    Byte(i8),
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
    pub fn from_be_bytes(bytes: &Vec<u8>) -> Tag {
        let mut fp = FileParser::new(bytes, 0);
        //set initial tag to compound as all data is implicitly in a compound
        Self::read_be(0x0A, &mut fp)
    }

    pub fn from_le_bytes(bytes: &Vec<u8>) -> Tag {
        let mut fp = FileParser::new(bytes, 0);
        //set initial tag to compound as all data is implicitly in a compound
        Self::read_le(0x0A, &mut fp)
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut fw = FileWriter::new();
        self.write_be(&mut fw);
        fw.bytes()
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut fw = FileWriter::new();
        self.write_be(&mut fw);
        fw.bytes()
    }

    fn tag_id(&self) -> u8 {
        match self {
            Tag::End => {0x00}
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

impl TagIo for Tag {
    fn read_be(tag_id: u8, fp: &mut FileParser) -> Tag {
        match tag_id {
            0x01 => {Tag::Byte(fp.read_i8())}
            0x02 => {Tag::Short(fp.read_be_i16())}
            0x03 => {Tag::Int(fp.read_be_i32())}
            0x04 => {Tag::Long(fp.read_be_i64())}
            0x05 => {Tag::Float(fp.read_be_f32())}
            0x06 => {Tag::Double(fp.read_be_f64())}
            0x07 => {
                let len = fp.read_be_i32();
                let mut array = Vec::<i8>::new();
                for _ in 0..len {
                    array.push(fp.read_i8());
                }
                Tag::ByteArray(array)
            }
            0x08 => {Tag::String(fp.read_be_var_string())}
            0x09 => {
                let type_id = fp.read_u8();
                let len = fp.read_be_i32();
                let mut array = Vec::<Tag>::new();
                for _ in 0..len {
                    array.push(Tag::read_be(type_id, fp));
                }
                Tag::List(array)
            }
            0x0A => {
                let mut buf = HashMap::<String, Tag>::new();
                while !fp.at_end() {
                    let tag_id = fp.read_u8();
                    if tag_id == 0x00 {
                        break;
                    }
                    buf.insert(fp.read_be_var_string(), Tag::read_be(tag_id, fp));
                }
                Tag::Compound(buf)
            }
            0x0B => {
                let len = fp.read_be_i32();
                let mut array = Vec::<i32>::new();
                for _ in 0..len {
                    array.push(fp.read_be_i32());
                }
                Tag::IntArray(array)
            }
            0x0C => {
                let len = fp.read_be_i64();
                let mut array = Vec::<i64>::new();
                for _ in 0..len {
                    array.push(fp.read_be_i64());
                }
                Tag::LongArray(array)
            }
            x => {
                panic!("Invalid Tag ID: {}", x)
            }
        }
    }

    fn read_le(tag_id: u8, fp: &mut FileParser) -> Tag {
        match tag_id {
            0x01 => {Tag::Byte(fp.read_i8())}
            0x02 => {Tag::Short(fp.read_le_i16())}
            0x03 => {Tag::Int(fp.read_le_i32())}
            0x04 => {Tag::Long(fp.read_le_i64())}
            0x05 => {Tag::Float(fp.read_le_f32())}
            0x06 => {Tag::Double(fp.read_le_f64())}
            0x07 => {
                let len = fp.read_le_i32();
                let mut array = Vec::<i8>::new();
                for _ in 0..len {
                    array.push(fp.read_i8());
                }
                Tag::ByteArray(array)
            }
            0x08 => {Tag::String(fp.read_le_var_string())}
            0x09 => {
                let type_id = fp.read_u8();
                let len = fp.read_le_i32();
                let mut array = Vec::<Tag>::new();
                for _ in 0..len {
                    array.push(Tag::read_le(type_id, fp));
                }
                Tag::List(array)
            }
            0x0A => {
                let mut buf = HashMap::<String, Tag>::new();
                while !fp.at_end() {
                    let tag_id = fp.read_u8();
                    if tag_id == 0x00 {
                        break;
                    }
                    buf.insert(fp.read_le_var_string(), Tag::read_le(tag_id, fp));
                }
                Tag::Compound(buf)
            }
            0x0B => {
                let len = fp.read_le_i32();
                let mut array = Vec::<i32>::new();
                for _ in 0..len {
                    array.push(fp.read_le_i32());
                }
                Tag::IntArray(array)
            }
            0x0C => {
                let len = fp.read_le_i64();
                let mut array = Vec::<i64>::new();
                for _ in 0..len {
                    array.push(fp.read_le_i64());
                }
                Tag::LongArray(array)
            }
            x => {
                panic!("Invalid Tag ID: {}", x)
            }
        }
    }

    fn write_be(&self, fw: &mut FileWriter) {
        match self {
            Tag::End => {}
            Tag::Byte(v) => {
                fw.write_i8(*v);
            }
            Tag::Short(v) => {
                fw.write_be_i16(*v);
            }
            Tag::Int(v) => {
                fw.write_be_i32(*v);
            }
            Tag::Long(v) => {
                fw.write_be_i64(*v);
            }
            Tag::Float(v) => {
                fw.write_be_f32(*v);
            }
            Tag::Double(v) => {
                fw.write_be_f64(*v);
            }
            Tag::ByteArray(v) => {
                fw.write_be_i32(v.len() as i32);
                for i in 0..v.len() {
                    fw.write_i8(v[i]);
                }
            }
            Tag::String(v) => {
                fw.write_be_var_string(v);
            }
            Tag::List(v) => {
                if v.len() == 0 {
                    fw.write_u8(0x00);
                } else {
                    fw.write_u8(v[0].tag_id());
                }
                fw.write_be_i32(v.len() as i32);
                for i in 0..v.len() {
                    v[i].write_be(fw);
                }
            }
            Tag::Compound(map) => {
                for (k, v) in map.iter() {
                    fw.write_u8(v.tag_id());
                    fw.write_be_var_string(k);
                    v.write_be(fw);
                }
                fw.write_u8(0x00);
            }
            Tag::IntArray(v) => {
                fw.write_be_i32(v.len() as i32);
                for i in 0..v.len() {
                    fw.write_be_i32(v[i]);
                }
            }
            Tag::LongArray(v) => {
                fw.write_be_i32(v.len() as i32);
                for i in 0..v.len() {
                    fw.write_be_i64(v[i]);
                }
            }
        }
    }

    fn write_le(&self, fw: &mut FileWriter) {
        match self {
            Tag::End => {}
            Tag::Byte(v) => {
                fw.write_i8(*v);
            }
            Tag::Short(v) => {
                fw.write_le_i16(*v);
            }
            Tag::Int(v) => {
                fw.write_le_i32(*v);
            }
            Tag::Long(v) => {
                fw.write_le_i64(*v);
            }
            Tag::Float(v) => {
                fw.write_le_f32(*v);
            }
            Tag::Double(v) => {
                fw.write_le_f64(*v);
            }
            Tag::ByteArray(v) => {
                fw.write_le_i32(v.len() as i32);
                for i in 0..v.len() {
                    fw.write_i8(v[i]);
                }
            }
            Tag::String(v) => {
                fw.write_le_var_string(v);
            }
            Tag::List(v) => {
                if v.len() == 0 {
                    fw.write_u8(0x00);
                } else {
                    fw.write_u8(v[0].tag_id());
                }
                fw.write_le_i32(v.len() as i32);
                for i in 0..v.len() {
                    v[i].write_be(fw);
                }
            }
            Tag::Compound(map) => {
                for (k, v) in map.iter() {
                    fw.write_u8(v.tag_id());
                    fw.write_le_var_string(k);
                    v.write_le(fw);
                }
                fw.write_u8(0x00);
            }
            Tag::IntArray(v) => {
                fw.write_le_i32(v.len() as i32);
                for i in 0..v.len() {
                    fw.write_le_i32(v[i]);
                }
            }
            Tag::LongArray(v) => {
                fw.write_le_i32(v.len() as i32);
                for i in 0..v.len() {
                    fw.write_le_i64(v[i]);
                }
            }
        }
    }
}