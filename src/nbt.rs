use crate::binary::{BinError, BinResult, FileReader, FileWriter, TagIo, Writer};
use std::{collections::HashMap, fmt::Debug};

#[repr(u8)]
#[derive(Clone, PartialEq, Debug)]
pub enum Tag {
    Byte(i8) = 1,
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(List),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum List {
    Empty,
    Byte(Vec<i8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<Vec<i8>>),
    String(Vec<String>),
    List(Vec<List>),
    Compound(Vec<HashMap<String, Tag>>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
}

impl List {
    pub fn len(&self) -> usize {
        match self {
            List::Empty => 0,
            List::Byte(list) => list.len(),
            List::Short(list) => list.len(),
            List::Int(list) => list.len(),
            List::Long(list) => list.len(),
            List::Float(list) => list.len(),
            List::Double(list) => list.len(),
            List::ByteArray(list) => list.len(),
            List::String(list) => list.len(),
            List::List(list) => list.len(),
            List::Compound(list) => list.len(),
            List::IntArray(list) => list.len(),
            List::LongArray(list) => list.len(),
        }
    }
}

impl Tag {
    fn tag_id(&self) -> u8 {
        match self {
            Tag::Byte(_) => 0x01,
            Tag::Short(_) => 0x02,
            Tag::Int(_) => 0x03,
            Tag::Long(_) => 0x04,
            Tag::Float(_) => 0x05,
            Tag::Double(_) => 0x06,
            Tag::ByteArray(_) => 0x07,
            Tag::String(_) => 0x08,
            Tag::List(_) => 0x09,
            Tag::Compound(_) => 0x0A,
            Tag::IntArray(_) => 0x0B,
            Tag::LongArray(_) => 0x0C,
        }
    }

    ///wraps the tag in a compound with it's key/name set to `k`
    pub fn wrapped(self, k: String) -> Self {
        let mut buf = HashMap::<String, Tag>::new();
        buf.insert(k, self);
        Tag::Compound(buf)
    }
}

macro_rules! read_array {
    ($fr:expr) => {{
        let len: i32 = $fr.read()?;
        let mut array = Vec::new();
        for _ in 0..len {
            array.push($fr.read()?);
        }
        array
    }};
}

macro_rules! read_list {
    ($len:expr, $fr:expr) => {{
        let mut array = Vec::new();
        for _ in 0..$len {
            array.push($fr.read()?);
        }
        array
    }};
}

macro_rules! read_list_array {
    ($len:expr, $fr:expr) => {{
        let mut array = Vec::new();
        for _ in 0..$len {
            let len_: i32 = $fr.read()?;
            let mut array_ = Vec::new();
            for _ in 0..len_ {
                array_.push($fr.read()?);
            }
            array.push(array_);
        }
        array
    }};
}

fn read_list(list_id: u8, fr: &mut impl FileReader) -> BinResult<List> {
    let len : i32 = fr.read()?;
    if len <= 0 {
        return Ok(List::Empty)
    }
    match list_id {
        0x00 => {
            if len > 0 {
                return Err(BinError::Parsing(
                        r#"Lists of type "Tag End" can't have a length greater than 0"#.to_string()
                ))
            }
            Ok(List::Empty)
        },
        0x01 => Ok(List::Byte(read_list!(len, fr))),
        0x02 => Ok(List::Short(read_list!(len, fr))),
        0x03 => Ok(List::Int(read_list!(len, fr))),
        0x04 => Ok(List::Long(read_list!(len, fr))),
        0x05 => Ok(List::Float(read_list!(len, fr))),
        0x06 => Ok(List::Double(read_list!(len, fr))),
        0x07 => Ok(List::ByteArray(read_list_array!(len, fr))),
        0x08 => Ok(List::String(read_list!(len, fr))),
        0x09 => {
            let len: i32 = fr.read()?;
            let mut array = Vec::new();
            for _ in 0..len {
                array.push(read_list(fr.read()?, fr)?);
            }
            Ok(List::List(array))
        }
        0x0A => {
            let len: i32 = fr.read()?;
            let mut array = Vec::new();
            for _ in 0..len {
                array.push(read_compound(fr)?);
            }
            Ok(List::Compound(array))
        }
        0x0B => Ok(List::IntArray(read_list_array!(len, fr))),
        0x0C => Ok(List::LongArray(read_list_array!(len, fr))),
        x => Err(BinError::Parsing(format!("Invalid Tag ID: {}", x))),
    }
}

fn read_compound(fr: &mut impl FileReader) -> BinResult<HashMap<String, Tag>> {
    let mut buf = HashMap::<String, Tag>::new();
    while !fr.at_end() {
        let tag_id: u8 = fr.read()?;
        if tag_id == 0x00 {
            break;
        }
        buf.insert(fr.read()?, Tag::read(tag_id, fr)?);
    }
    Ok(buf)
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
            0x07 => Ok(Tag::ByteArray(read_array!(fr))),
            0x08 => Ok(Tag::String(fr.read()?)),
            0x09 => Ok(Tag::List(read_list(fr.read()?, fr)?)),
            0x0A => Ok(Tag::Compound(read_compound(fr)?)),
            0x0B => Ok(Tag::IntArray(read_array!(fr))),
            0x0C => Ok(Tag::LongArray(read_array!(fr))),
            x => Err(BinError::Parsing(format!("Invalid Tag ID: {}", x))),
        }
    }
}

macro_rules! write_array {
    ($v:expr, $fw:expr) => {{
        $fw.write(&($v.len() as i32));
        for i in 0..$v.len() {
            $fw.write(&$v[i]);
        }
    }};
}

macro_rules! write_list {
    ($id:literal, $v:expr, $fw:expr) => {{
        $fw.write(&$id);
        write_array!($v, $fw);
    }};
}

macro_rules! write_array_list {
    ($id:literal, $v:expr, $fw:expr) => {{
        $fw.write(&$id);
        $fw.write(&($v.len() as i32));
        for i in 0..$v.len() {
            let w = &$v[i];
            $fw.write(&(w.len() as i32));
            for j in 0..w.len() {
                $fw.write(&w[j]);
            }
        }
    }};
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
            Tag::ByteArray(v) => write_array!(v, fw),
            Tag::String(v) => fw.write(v),
            Tag::List(v) => fw.write(v),
            Tag::Compound(map) => {
                for (k, v) in map.iter() {
                    fw.write(&v.tag_id());
                    fw.write(k);
                    fw.write(v);
                }
                fw.write::<u8>(&0x00);
            }
            Tag::IntArray(v) => write_array!(v, fw),
            Tag::LongArray(v) => write_array!(v, fw),
        }
    }
}

impl Writer for List {
    fn write(&self, fw: &mut impl FileWriter) {
        // use tag id `0x00` if length is 0
        if self.len() == 0 {
            fw.append(&mut vec![0x00; 5])
        } else {
            match self {
                List::Empty => fw.append(&mut vec![0x00; 5]),
                List::Byte(arr) => write_list!(0x01, arr, fw),
                List::Short(arr) => write_list!(0x02, arr, fw),
                List::Int(arr) => write_list!(0x03, arr, fw),
                List::Long(arr) => write_list!(0x04, arr, fw),
                List::Float(arr) => write_list!(0x05, arr, fw),
                List::Double(arr) => write_list!(0x06, arr, fw),
                List::ByteArray(arr) => write_array_list!(0x07, arr, fw),
                List::String(arr) => write_list!(0x08, arr, fw),
                List::List(arr) => write_list!(0x09, arr, fw),
                List::Compound(arr) => {
                    fw.write::<u8>(&0x0A);
                    fw.write(&(arr.len() as i32));
                    for i in 0..arr.len() {
                        let map = &arr[i];
                        for (k, v) in map.iter() {
                            fw.write(&v.tag_id());
                            fw.write(k);
                            fw.write(v);
                        }
                        fw.write::<u8>(&0x00);
                    }
                }
                List::IntArray(arr) => write_array_list!(0x0B, arr, fw),
                List::LongArray(arr) => write_array_list!(0x0B, arr, fw),
            }
        }
    }
}
