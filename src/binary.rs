use mutf8::MString;
use core::array::TryFromSliceError;

pub type BinResult<T> = std::result::Result<T, BinError>;

#[derive(Debug)]
pub enum BinError {
    UnexpectedEndOfByteStream,
    Parse(TryFromSliceError),
}

pub trait Writer {
    fn write(&self, fw: &mut impl FileWriter);
}

pub trait Io: Writer {
    fn read(fr: &mut impl FileReader) -> BinResult<Self> where Self: Sized;
}

pub trait PrimitiveIo: Io {
    fn primitive_read_be(fr: &mut impl FileReader) -> BinResult<Self> where Self: Sized;
    fn primitive_read_le(fr: &mut impl FileReader) -> BinResult<Self> where Self: Sized;
    fn primitive_write_be(&self, fw: &mut impl FileWriter);
    fn primitive_write_le(&self, fw: &mut impl FileWriter);
}

macro_rules! io_primitive {
    ($type:tt, $size:literal) => {
        impl PrimitiveIo for $type {
            fn primitive_read_be(fr: &mut impl FileReader) -> BinResult<Self> {
                let bytes = fr.get_slice($size)?;
                let r: Result<[u8; $size], TryFromSliceError> = bytes.try_into();
                match r {
                    Err(e) => Err(BinError::Parse(e)),
                    Ok(x) => Ok(Self::from_be_bytes(x))
                }
            }

            fn primitive_read_le(fr: &mut impl FileReader) -> BinResult<Self> {
                let bytes = fr.get_slice($size)?;
                let r: Result<[u8; $size], TryFromSliceError> = bytes.try_into();
                match r {
                    Err(e) => Err(BinError::Parse(e)),
                    Ok(x) => Ok(Self::from_le_bytes(x))
                }
            }

            fn primitive_write_be(&self, fw: &mut impl FileWriter) {
                fw.append(&mut Self::to_be_bytes(*self).to_vec())
            }
        
            fn primitive_write_le(&self, fw: &mut impl FileWriter) {
                fw.append(&mut Self::to_le_bytes(*self).to_vec())
            }
        }

        impl Io for $type {
            fn read(fr: &mut impl FileReader) -> BinResult<Self> {
                fr.primitive_read()
            }
        }

        impl Writer for $type {
            fn write(&self, fw: &mut impl FileWriter) {
                fw.primitive_write(self)
            }
        }
    };
}

io_primitive!(u8, 1);
io_primitive!(i8, 1);
io_primitive!(u16, 2);
io_primitive!(i16, 2);
io_primitive!(u32, 4);
io_primitive!(i32, 4);
io_primitive!(u64, 8);
io_primitive!(i64, 8);
io_primitive!(f32, 4);
io_primitive!(f64, 8);

impl Io for String {
    fn read(fr: &mut impl FileReader) -> BinResult<Self> {
        let len = fr.read::<u16>()? as usize;
        Ok(MString::from_mutf8(fr.get_slice(len)?).to_string())
    }
}

impl Writer for String {
    fn write(&self, fw: &mut impl FileWriter) {
        fw.write(&(self.len() as u16));
        fw.append(&mut MString::from_utf8(
            String::as_bytes(self)
        ).unwrap().as_mutf8_bytes().to_vec());
    }
}


pub trait TagIo: Writer {
    fn read(tag_id: u8, fr: &mut impl FileReader) -> BinResult<Self> where Self: Sized;
}

pub trait FileReader: PrimitiveFileReader {
    ///reads a data type in the endianness the file reader is set to
    fn read<T: Io>(&mut self) -> BinResult<T> where Self: Sized {
        T::read(self)
    }
    ///reads a data type in ``big endian``
    fn read_be<T: Io>(&mut self) -> BinResult<T> where Self: Sized;
    ///reads a data type in ``little endian``
    fn read_le<T: Io>(&mut self) -> BinResult<T> where Self: Sized;
    fn get_slice(&mut self, len: usize) -> BinResult<&[u8]>;
    fn rest(&self) -> Vec<u8>;
    fn at_end(&self) -> bool;
}

pub trait PrimitiveFileReader {
    fn primitive_read<T: PrimitiveIo>(&mut self) -> BinResult<T> where Self: Sized;
    fn primitive_read_be<T: PrimitiveIo>(&mut self) -> BinResult<T> where Self: Sized, Self: FileReader {
        T::primitive_read_be(self)
    }
    fn primitive_read_le<T: PrimitiveIo>(&mut self) -> BinResult<T> where Self: Sized, Self: FileReader {
        T::primitive_read_le(self)
    }
}

macro_rules! file_reader {
    ($reader:ident, $endian:ident, $reader_inverse:ident, $endian_inverse:ident, $endian_primitive:ident) => {
        pub struct $reader<'a> {
            bytes: &'a Vec<u8>,
            pos: usize
        }
        
        impl<'a> $reader<'a> {
            pub fn new(bytes: &'a Vec<u8>, pos: usize) -> Self {
                Self {
                    bytes,
                    pos
                }
            }
        }

        impl<'a> PrimitiveFileReader for $reader<'a> {
            fn primitive_read<T: PrimitiveIo>(&mut self) -> BinResult<T> {
                T::$endian_primitive(self)
            }
        }
        
        impl<'a> FileReader for $reader<'a> {
            fn $endian<T: Io>(&mut self) -> BinResult<T> {
                T::read(self)
            }

            fn $endian_inverse<T: Io>(&mut self) -> BinResult<T> {
                let mut inverse = $reader_inverse::new(self.bytes, self.pos);
                let r = T::read(&mut inverse);
                self.pos = inverse.pos;
                r
            }
        
            fn get_slice(&mut self, len: usize) -> Result<&[u8], BinError> {
                self.pos += len;
                if self.pos > self.bytes.len() {
                    return Err(BinError::UnexpectedEndOfByteStream)
                }
                Ok(&self.bytes[self.pos-len..self.pos])
            }
        
            fn rest(&self) -> Vec<u8> {
                self.bytes[self.pos..].to_owned()
            }
        
            fn at_end(&self) -> bool {
                self.pos == self.bytes.len()
            }
        }
    }
}

file_reader!(FileReaderBE, read_be, FileReaderLE, read_le, primitive_read_be);
file_reader!(FileReaderLE, read_le, FileReaderBE, read_be, primitive_read_le);

pub trait FileWriter: PrimitiveFileWriter {
    fn write<T: Writer>(&mut self, v: &T) where Self: Sized {
        v.write(self);
    }

    fn write_be<T: Writer>(&mut self, v: &T);

    fn write_le<T: Writer>(&mut self, v: &T);

    fn append(&mut self, bytes: &mut Vec::<u8>);

    fn bytes(self) -> Vec<u8>;
}

pub trait PrimitiveFileWriter {
    fn primitive_write<T: PrimitiveIo>(&mut self, v: &T);

    fn primitive_write_be<T: PrimitiveIo>(&mut self, v: &T) where Self: Sized, Self: FileWriter {
        v.primitive_write_be(self);
    }

    fn primitive_write_le<T: PrimitiveIo>(&mut self, v: &T) where Self: Sized, Self: FileWriter {
        v.primitive_write_le(self);
    }
}

macro_rules! file_writer {
    ($writer:ident, $endian:ident, $writer_inverse:ident, $endian_inverse:ident, $endian_primitive:ident) => {
        pub struct $writer {
            bytes: Vec<u8>
        }

        impl $writer {
            pub fn new() -> Self {
                Self {
                    bytes: Vec::<u8>::new()
                }
            }
        }

        impl PrimitiveFileWriter for $writer {
            fn primitive_write<T: PrimitiveIo>(&mut self, v: &T) {
                v.$endian_primitive(self);
            }
        }

        impl FileWriter for $writer {
            fn $endian<T: Writer>(&mut self, v: &T) {
                v.write(self)
            }

            fn $endian_inverse<T: Writer>(&mut self, v: &T) {
                let mut inverse = $writer_inverse::new();
                v.write(&mut inverse);
                self.append(&mut inverse.bytes);
            }
        
            fn append(&mut self, bytes: &mut Vec::<u8>) {
                self.bytes.append(bytes);
            }
        
            fn bytes(self) -> Vec<u8> {
                self.bytes
            }
        }
    }
}

file_writer!(FileWriterBE, write_be, FileWriterLE, write_le, primitive_write_be);
file_writer!(FileWriterLE, write_le, FileWriterBE, write_be, primitive_write_le);