use mutf8::MString;
use std::io::{Read, Write};
use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};

pub const GZIP_HEADER: [u8; 2] = [0x1F, 0x8B];
pub const ZLIB_HEADER: [u8; 1] = [0x78];

pub enum Compression {
    Uncompressed,
    GZIP,
    ZLIB
}

impl Compression {
    pub fn decode(&self, buf: Vec<u8>) -> std::io::Result<Vec<u8>> {
        match self {
            Compression::Uncompressed => {Ok(buf)}
            Compression::GZIP => {
                let mut data = vec![];
                GzDecoder::new(&buf[..]).read_to_end(&mut data)?;
                Ok(data)
            }
            Compression::ZLIB => {
                let mut data = vec![];
                ZlibDecoder::new(&buf[..]).read_to_end(&mut data)?;
                Ok(data)
            }
        }
    }

    pub fn encode(&self, buf: Vec<u8>) -> std::io::Result<Vec<u8>> {
        match self {
            Compression::Uncompressed => {Ok(buf)}
            Compression::GZIP => {
                let mut encoder = GzEncoder::new(
                    Vec::new(),
                    flate2::Compression::default()
                );
                encoder.write_all(&buf)?;
                Ok(encoder.finish().unwrap().to_vec())
            }
            Compression::ZLIB => {
                let mut encoder = ZlibEncoder::new(
                    Vec::new(),
                    flate2::Compression::default()
                );
                encoder.write_all(&buf)?;
                Ok(encoder.finish().unwrap().to_vec())
            }
        }
    }

    pub fn as_str(&self) -> &str {
        match &self {
            Compression::Uncompressed => {"uncompressed"}
            Compression::GZIP => {"gzip"}
            Compression::ZLIB => {"zlib"}
        }
    }
}

pub trait TagIo {
    fn read_be(tag_id: u8, fp: &mut FileParser) -> crate::nbt::Tag;
    fn read_le(tag_id: u8, fp: &mut FileParser) -> crate::nbt::Tag;
    fn write_be(&self, fw: &mut FileWriter);
    fn write_le(&self, fw: &mut FileWriter);
}

pub struct FileParser<'a> {
    bytes: &'a Vec<u8>,
    pos: usize
}

impl<'a> FileParser<'a> {
    pub fn new(bytes: &Vec<u8>, pos: usize) -> FileParser {
        FileParser {
            bytes,
            pos
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        self.pos += 1;
        u8::from_be_bytes(self.bytes[self.pos-1..self.pos].try_into().unwrap())
    }

    pub fn read_i8(&mut self) -> i8 {
        self.pos += 1;
        i8::from_be_bytes(self.bytes[self.pos-1..self.pos].try_into().unwrap())
    }

    pub fn read_be_u16(&mut self) -> u16 {
        self.pos += 2;
        u16::from_be_bytes(self.bytes[self.pos-2..self.pos].try_into().unwrap())
    }

    pub fn read_be_i16(&mut self) -> i16 {
        self.pos += 2;
        i16::from_be_bytes(self.bytes[self.pos-2..self.pos].try_into().unwrap())
    }

    pub fn read_be_i32(&mut self) -> i32 {
        self.pos += 4;
        i32::from_be_bytes(self.bytes[self.pos-4..self.pos].try_into().unwrap())
    }

    pub fn read_be_i64(&mut self) -> i64 {
        self.pos += 8;
        i64::from_be_bytes(self.bytes[self.pos-8..self.pos].try_into().unwrap())
    }

    pub fn read_be_f32(&mut self) -> f32 {
        self.pos += 4;
        f32::from_be_bytes(self.bytes[self.pos-4..self.pos].try_into().unwrap())
    }

    pub fn read_be_f64(&mut self) -> f64 {
        self.pos += 8;
        f64::from_be_bytes(self.bytes[self.pos-8..self.pos].try_into().unwrap())
    }

    pub fn read_be_var_string(&mut self) -> String {
        let len = u16::from_be_bytes(
            self.bytes[self.pos..self.pos+2].try_into().unwrap()
        ) as usize;
        self.pos += len + 2;
        MString::from_mutf8(&self.bytes[self.pos-len..self.pos]).to_string()
    }

    pub fn read_le_u16(&mut self) -> u16 {
        self.pos += 2;
        u16::from_le_bytes(self.bytes[self.pos-2..self.pos].try_into().unwrap())
    }

    pub fn read_le_i16(&mut self) -> i16 {
        self.pos += 2;
        i16::from_le_bytes(self.bytes[self.pos-2..self.pos].try_into().unwrap())
    }

    pub fn read_le_i32(&mut self) -> i32 {
        self.pos += 4;
        i32::from_le_bytes(self.bytes[self.pos-4..self.pos].try_into().unwrap())
    }

    pub fn read_le_i64(&mut self) -> i64 {
        self.pos += 8;
        i64::from_le_bytes(self.bytes[self.pos-8..self.pos].try_into().unwrap())
    }

    pub fn read_le_f32(&mut self) -> f32 {
        self.pos += 4;
        f32::from_le_bytes(self.bytes[self.pos-4..self.pos].try_into().unwrap())
    }

    pub fn read_le_f64(&mut self) -> f64 {
        self.pos += 8;
        f64::from_le_bytes(self.bytes[self.pos-8..self.pos].try_into().unwrap())
    }

    pub fn read_le_var_string(&mut self) -> String {
        let len = u16::from_le_bytes(
            self.bytes[self.pos..self.pos+2].try_into().unwrap()
        ) as usize;
        self.pos += len + 2;
        MString::from_mutf8(&self.bytes[self.pos-len..self.pos]).to_string()
    }

    pub fn rest(&self) -> Vec<u8> {
        self.bytes[self.pos..].to_owned()
    }

    pub fn at_end(&self) -> bool {
        self.pos == self.bytes.len()
    }
}

pub struct FileWriter {
    bytes: Vec<u8>
}

impl FileWriter {
    pub fn new() -> FileWriter {
        FileWriter {
            bytes: Vec::<u8>::new()
        }
    }

    pub fn append(&mut self, bytes: &mut Vec::<u8>) {
        self.bytes.append(bytes);
    }

    pub fn write_u8(&mut self, v: u8) {
        self.bytes.append(&mut u8::to_be_bytes(v).to_vec());
    }

    pub fn write_i8(&mut self, v: i8) {
        self.bytes.append(&mut i8::to_be_bytes(v).to_vec())
    }

    pub fn write_be_u16(&mut self, v: u16) {
        self.bytes.append(&mut u16::to_be_bytes(v).to_vec());
    }

    pub fn write_be_i16(&mut self, v: i16) {
        self.bytes.append(&mut i16::to_be_bytes(v).to_vec())
    }

    pub fn write_be_i32(&mut self, v: i32) {
        self.bytes.append(&mut i32::to_be_bytes(v).to_vec())
    }

    pub fn write_be_i64(&mut self, v: i64) {
        self.bytes.append(&mut i64::to_be_bytes(v).to_vec())
    }

    pub fn write_be_f32(&mut self, v: f32) {
        self.bytes.append(&mut f32::to_be_bytes(v).to_vec())
    }

    pub fn write_be_f64(&mut self, v: f64) {
        self.bytes.append(&mut f64::to_be_bytes(v).to_vec())
    }

    pub fn write_be_var_string(&mut self, v: &String) {
        self.bytes.append(&mut u16::to_be_bytes(v.len() as u16).to_vec());
        self.bytes.append(&mut MString::from_utf8(
            String::as_bytes(v)).unwrap().as_mutf8_bytes().to_vec()
        );
    }

    pub fn write_le_u16(&mut self, v: u16) {
        self.bytes.append(&mut u16::to_be_bytes(v).to_vec());
    }

    pub fn write_le_i16(&mut self, v: i16) {
        self.bytes.append(&mut i16::to_be_bytes(v).to_vec())
    }

    pub fn write_le_i32(&mut self, v: i32) {
        self.bytes.append(&mut i32::to_be_bytes(v).to_vec())
    }

    pub fn write_le_i64(&mut self, v: i64) {
        self.bytes.append(&mut i64::to_be_bytes(v).to_vec())
    }

    pub fn write_le_f32(&mut self, v: f32) {
        self.bytes.append(&mut f32::to_be_bytes(v).to_vec())
    }

    pub fn write_le_f64(&mut self, v: f64) {
        self.bytes.append(&mut f64::to_be_bytes(v).to_vec())
    }

    pub fn write_le_var_string(&mut self, v: &String) {
        self.bytes.append(&mut u16::to_be_bytes(v.len() as u16).to_vec());
        self.bytes.append(&mut MString::from_utf8(
            String::as_bytes(v)).unwrap().as_mutf8_bytes().to_vec()
        );
    }

    pub fn bytes(self) -> Vec<u8> {
        self.bytes
    }
}