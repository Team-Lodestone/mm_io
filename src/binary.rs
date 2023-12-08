use mutf8::MString;
use std::io::{Read, Write};
use core::array::TryFromSliceError;
use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};

pub const GZIP_MAGIC_NUMBER: [u8; 2] = [0x1F, 0x8B];
pub const ZLIB_MAGIC_NUMBER: [u8; 1] = [0x78];

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

    pub fn magic_number(&self) -> &[u8] {
        match &self {
            Compression::Uncompressed => {&[]}
            Compression::GZIP => {&GZIP_MAGIC_NUMBER}
            Compression::ZLIB => {&ZLIB_MAGIC_NUMBER}
        }
    }
}

pub type BinResult<T> = std::result::Result<T, BinError>;

#[derive(Debug)]
pub enum BinError {
    UnexpectedEndOfByteStream,
    Parse(TryFromSliceError),
}

pub trait Writer {
    fn write_be(&self, fw: &mut FileWriter);
    fn write_le(&self, fw: &mut FileWriter);
}

pub trait Io: Writer {
    fn read_be(fr: &mut FileReader) -> BinResult<Self> where Self: Sized;
    fn read_le(fr: &mut FileReader) -> BinResult<Self> where Self: Sized;
}

#[macro_export]
macro_rules! io_static_size {
    ($type:tt, $size:literal) => {
        impl Io for $type {
            fn read_be(fr: &mut FileReader) -> BinResult<Self> {
                let bytes = fr.get_slice($size)?;
                let r: Result<[u8; $size], TryFromSliceError> = bytes.try_into();
                match r {
                    Err(e) => Err(BinError::Parse(e)),
                    Ok(x) => Ok(Self::from_be_bytes(x))
                }
            }

            fn read_le(fr: &mut FileReader) -> BinResult<Self> {
                let bytes = fr.get_slice($size)?;
                let r: Result<[u8; $size], TryFromSliceError> = bytes.try_into();
                match r {
                    Err(e) => Err(BinError::Parse(e)),
                    Ok(x) => Ok(Self::from_le_bytes(x))
                }
            }
        }

        impl Writer for $type {
            fn write_be(&self, fw: &mut FileWriter) {
                fw.append(&mut Self::to_be_bytes(*self).to_vec())
            }
        
            fn write_le(&self, fw: &mut FileWriter) {
                fw.append(&mut Self::to_le_bytes(*self).to_vec())
            }
        }
    };
}

io_static_size!(u8, 1);
io_static_size!(i8, 1);
io_static_size!(u16, 2);
io_static_size!(i16, 2);
io_static_size!(i32, 4);
io_static_size!(i64, 8);
io_static_size!(f32, 4);
io_static_size!(f64, 8);

impl Io for String {
    fn read_be(fr: &mut FileReader) -> BinResult<Self> where Self: Sized {
        let len = fr.read_be::<u16>()? as usize;
        Ok(MString::from_mutf8(fr.get_slice(len)?).to_string())
    }

    fn read_le(fr: &mut FileReader) -> BinResult<Self> where Self: Sized {
        let len = fr.read_le::<u16>()? as usize;
        Ok(MString::from_mutf8(fr.get_slice(len)?).to_string())
    }
}

impl Writer for String {
    fn write_be(&self, fw: &mut FileWriter) {
        fw.write_be(&(self.len() as u16));
        fw.append(&mut MString::from_utf8(
            String::as_bytes(self)
        ).unwrap().as_mutf8_bytes().to_vec());
    }

    fn write_le(&self, fw: &mut FileWriter) {
        fw.write_le(&(self.len() as u16));
        fw.append(&mut MString::from_utf8(
            String::as_bytes(self)
        ).unwrap().as_mutf8_bytes().to_vec());
    }
}


pub trait TagIo: Writer {
    fn read_be(tag_id: u8, fr: &mut FileReader) -> BinResult<Self> where Self: Sized;
    fn read_le(tag_id: u8, fr: &mut FileReader) -> BinResult<Self> where Self: Sized;
}

pub struct FileReader<'a> {
    bytes: &'a Vec<u8>,
    pos: usize
}

impl<'a> FileReader<'a> {
    pub fn new(bytes: &Vec<u8>, pos: usize) -> FileReader {
        FileReader {
            bytes,
            pos
        }
    }

    pub fn read_be<T: Io>(&mut self) -> BinResult<T> {
        T::read_be(self)
    }

    pub fn read_le<T: Io>(&mut self) -> BinResult<T> {
        T::read_le(self)
    }

    pub fn get_slice(&mut self, len: usize) -> Result<&[u8], BinError> {
        self.pos += len;
        if self.pos > self.bytes.len() {
            return Err(BinError::UnexpectedEndOfByteStream)
        }
        Ok(&self.bytes[self.pos-len..self.pos])
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

    pub fn write_be<T: Writer>(&mut self, v: &T) {
        v.write_be(self);
    }

    pub fn write_le<T: Writer>(&mut self, v: &T) {
        v.write_le(self);
    }

    pub fn append(&mut self, bytes: &mut Vec::<u8>) {
        self.bytes.append(bytes);
    }

    pub fn bytes(self) -> Vec<u8> {
        self.bytes
    }
}