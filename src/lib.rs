pub mod binary;
pub mod nbt;

#[cfg(test)]
mod tests {
    use super::*;
    use binary as bin;
    //allows for reading/writing tag payloads with read_be & write_be
    use bin::TagIo;

    #[test]
    fn read_u8() {
        {
            let x = &vec![0x01];
            let mut fr = bin::FileReader::new(x, 0);
            let byte = fr.read_be::<u8>().unwrap();
            assert_eq!(byte, 0x01);
        }
    }

    #[test]
    fn read_be_byte_tag() {
        {
            //---payloads---//
            let x = &vec![
                0x00, 
                0x01, 
                0x02, 
                0x03
            ];
            let mut fr = bin::FileReader::new(x, 0);
            let tag_id = 0x01;
            let tag_0 = nbt::Tag::read_be(tag_id, &mut fr).unwrap();
            assert_eq!(tag_0, nbt::Tag::Byte(0x00));
            let tag_1 = nbt::Tag::read_be(tag_id, &mut fr).unwrap();
            assert_eq!(tag_1, nbt::Tag::Byte(0x01));
            let tag_2 = nbt::Tag::read_be(tag_id, &mut fr).unwrap();
            assert_eq!(tag_2, nbt::Tag::Byte(0x02));
            let tag_3 = nbt::Tag::read_be(tag_id, &mut fr).unwrap();
            assert_eq!(tag_3, nbt::Tag::Byte(0x03));
        }
    }

    #[test]
    fn read_le_byte_tag() {
        {
            //---payloads---//
            let x = &vec![
                0x00, 
                0x01, 
                0x02, 
                0x03
            ];
            let mut fr = bin::FileReader::new(x, 0);
            let tag_id = 0x01;
            let tag_0 = nbt::Tag::read_le(tag_id, &mut fr).unwrap();
            assert_eq!(tag_0, nbt::Tag::Byte(0x00));
            let tag_1 = nbt::Tag::read_le(tag_id, &mut fr).unwrap();
            assert_eq!(tag_1, nbt::Tag::Byte(0x01));
            let tag_2 = nbt::Tag::read_le(tag_id, &mut fr).unwrap();
            assert_eq!(tag_2, nbt::Tag::Byte(0x02));
            let tag_3 = nbt::Tag::read_le(tag_id, &mut fr).unwrap();
            assert_eq!(tag_3, nbt::Tag::Byte(0x03));
        }
    }

    #[test]
    fn read_be_short_tag() {
        {
            //---payloads---//
            let x = &vec![
                0x00, 0x04,
                0x01, 0x05,
                0x02, 0x06,
                0x03, 0x07
            ];
            let mut fr = bin::FileReader::new(x, 0);
            let tag_id = 0x02;
            let tag_0 = nbt::Tag::read_be(tag_id, &mut fr).unwrap();
            assert_eq!(tag_0, nbt::Tag::Short(0x0004));
            let tag_1 = nbt::Tag::read_be(tag_id, &mut fr).unwrap();
            assert_eq!(tag_1, nbt::Tag::Short(0x0105));
            let tag_2 = nbt::Tag::read_be(tag_id, &mut fr).unwrap();
            assert_eq!(tag_2, nbt::Tag::Short(0x0206));
            let tag_3 = nbt::Tag::read_be(tag_id, &mut fr).unwrap();
            assert_eq!(tag_3, nbt::Tag::Short(0x0307));
        }
    }

    #[test]
    fn read_le_short_tag() {
        {
            //---payloads---//
            let x = &vec![
                0x00, 0x04,
                0x01, 0x05,
                0x02, 0x06,
                0x03, 0x07
            ];
            let mut fr = bin::FileReader::new(x, 0);
            let tag_id = 0x02;
            let tag_0 = nbt::Tag::read_le(tag_id, &mut fr).unwrap();
            assert_eq!(tag_0, nbt::Tag::Short(0x0400));
            let tag_1 = nbt::Tag::read_le(tag_id, &mut fr).unwrap();
            assert_eq!(tag_1, nbt::Tag::Short(0x0501));
            let tag_2 = nbt::Tag::read_le(tag_id, &mut fr).unwrap();
            assert_eq!(tag_2, nbt::Tag::Short(0x0602));
            let tag_3 = nbt::Tag::read_le(tag_id, &mut fr).unwrap();
            assert_eq!(tag_3, nbt::Tag::Short(0x0703));
        }
    }
}
