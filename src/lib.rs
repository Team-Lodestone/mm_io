pub mod binary;
pub mod compression;
pub mod nbt;

#[cfg(test)]
mod tests {
    use super::*;
    use bin::FileReader;
    use binary as bin;
    //allows for reading/writing tag payloads with read_be & write_be
    use bin::TagIo;

    #[test]
    fn read_u8() {
        {
            let x = &vec![0x01];
            let mut fr = bin::FileReaderBE::new(x, 0);
            let byte = fr.read::<u8>().unwrap();
            assert_eq!(byte, 0x01);
        }
    }

    #[test]
    fn read_be_byte_tag() {
        //---payloads---//
        let x = &vec![0x00, 0x01, 0x02, 0x03];
        let tag_id = 0x01;
        let mut fr = bin::FileReaderBE::new(x, 0);
        assert_eq!(
            nbt::Tag::read(tag_id, &mut fr).unwrap(),
            nbt::Tag::Byte(0x00)
        );
        assert_eq!(
            nbt::Tag::read(tag_id, &mut fr).unwrap(),
            nbt::Tag::Byte(0x01)
        );
        assert_eq!(
            nbt::Tag::read(tag_id, &mut fr).unwrap(),
            nbt::Tag::Byte(0x02)
        );
        assert_eq!(
            nbt::Tag::read(tag_id, &mut fr).unwrap(),
            nbt::Tag::Byte(0x03)
        );
    }

    #[test]
    fn read_le_byte_tag() {
        //---payloads---//
        let x = &vec![0x00, 0x01, 0x02, 0x03];
        let tag_id = 0x01;
        let mut fr = bin::FileReaderLE::new(x, 0);
        assert_eq!(
            nbt::Tag::read(tag_id, &mut fr).unwrap(),
            nbt::Tag::Byte(0x00)
        );
        assert_eq!(
            nbt::Tag::read(tag_id, &mut fr).unwrap(),
            nbt::Tag::Byte(0x01)
        );
        assert_eq!(
            nbt::Tag::read(tag_id, &mut fr).unwrap(),
            nbt::Tag::Byte(0x02)
        );
        assert_eq!(
            nbt::Tag::read(tag_id, &mut fr).unwrap(),
            nbt::Tag::Byte(0x03)
        );
    }

    #[test]
    fn read_be_short_tag() {
        {
            //---payloads---//
            let x = &vec![0x00, 0x04, 0x01, 0x05, 0x02, 0x06, 0x03, 0x07];
            let tag_id = 0x02;
            let mut fr = bin::FileReaderBE::new(x, 0);
            assert_eq!(
                nbt::Tag::read(tag_id, &mut fr).unwrap(),
                nbt::Tag::Short(0x0004)
            );
            assert_eq!(
                nbt::Tag::read(tag_id, &mut fr).unwrap(),
                nbt::Tag::Short(0x0105)
            );
            assert_eq!(
                nbt::Tag::read(tag_id, &mut fr).unwrap(),
                nbt::Tag::Short(0x0206)
            );
            assert_eq!(
                nbt::Tag::read(tag_id, &mut fr).unwrap(),
                nbt::Tag::Short(0x0307)
            );
        }
    }

    #[test]
    fn read_le_short_tag() {
        {
            //---payloads---//
            let x = &vec![0x00, 0x04, 0x01, 0x05, 0x02, 0x06, 0x03, 0x07];
            let tag_id = 0x02;
            let mut fr = bin::FileReaderLE::new(x, 0);
            assert_eq!(
                nbt::Tag::read(tag_id, &mut fr).unwrap(),
                nbt::Tag::Short(0x0400)
            );
            assert_eq!(
                nbt::Tag::read(tag_id, &mut fr).unwrap(),
                nbt::Tag::Short(0x0501)
            );
            assert_eq!(
                nbt::Tag::read(tag_id, &mut fr).unwrap(),
                nbt::Tag::Short(0x0602)
            );
            assert_eq!(
                nbt::Tag::read(tag_id, &mut fr).unwrap(),
                nbt::Tag::Short(0x0703)
            );
        }
    }
}
