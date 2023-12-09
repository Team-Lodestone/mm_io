use std::io::{Read, Write};
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