#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod encoding {
    mod binaryreader;
    pub use binaryreader::{BinaryReader, DataDecodeError, InvalidDataError};
    mod binarywriter;
    pub use binarywriter::BinaryWriter;
}
pub use encoding::{BinaryReader, DataDecodeError, InvalidDataError};
pub use encoding::BinaryWriter;


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;
    use crate::encoding::DataDecodeError;

    const TEST_FOLDER: &str = "csharp_testing";

    #[test]
    fn decode_all_types() -> Result<(), DataDecodeError>{
        use std::fs::File;
        use xshell::{Shell, cmd};
        let sh = Shell::new().unwrap();
        sh.change_dir(&TEST_FOLDER.to_string());
        cmd!(sh, "dotnet run generate").run().unwrap();

        let file = File::open(TEST_FOLDER.to_string()+ "/output.bin")?;
        let mut reader = BinaryReader::new(file);

        // read the test data written in generate_test_bin.cs
        assert!(reader.read_boolean()?);
        assert!(!reader.read_boolean()?);
        assert_eq!(0x45, reader.read_byte()?);
        assert_eq!(vec![0x01, 0x02, 0x03, 0x04, 0x05], reader.read_bytes(5)?);
        assert_eq!('\u{2603}' as char, reader.read_char()?);
        assert_eq!(727.247_f64, reader.read_f64()?);
        cfg_if::cfg_if! {
            if #[cfg(feature = "f16")] {
                assert_eq!(247_f16, reader.read_f16()?);
            } else {
                reader.read_bytes(2)?; // just skip the two bytes instead
            }
        }
        assert_eq!(-5_i16, reader.read_i16()?);
        assert_eq!(-100_i32, reader.read_i32()?);
        assert_eq!(-2147483649_i64, reader.read_i64()?);
        assert_eq!(-112_i8, reader.read_i8()?);
        assert_eq!(5.2_f32, reader.read_f32()?);
        assert_eq!(-723_i32, reader.read_7_bit_encoded_int()?);
        assert_eq!(404_i32, reader.read_7_bit_encoded_int()?);
        assert_eq!(9000000000000000000_i64, reader.read_7_bit_encoded_int64()?);
        assert_eq!(-500000000000000000_i64, reader.read_7_bit_encoded_int64()?);
        assert_eq!("meowmeowmeowmeowmeow".to_string(), reader.read_string()?);
        assert_eq!(624_u16, reader.read_u16()?);
        assert_eq!(3000000000_u32, reader.read_u32()?);
        assert_eq!(42307830165_u64, reader.read_u64()?);
        assert_eq!('\0', reader.read_char()?);

        let _ = cmd!(sh, "rm -f output.bin").run();

        Ok(())
    }

    #[test]
    fn encode_all_types(){
        use std::fs::File;
        use xshell::{Shell, cmd};
        
        let sh = Shell::new().unwrap();
        sh.change_dir(&TEST_FOLDER.to_string());

        let _ = cmd!(sh, "rm -f input.bin").run().unwrap();

        let mut file = File::create(TEST_FOLDER.to_string() + "/input.bin").unwrap();
        let mut writer = BinaryWriter::new(&mut file);

        writer.write_boolean(true).unwrap();
        writer.write_boolean(false).unwrap();
        writer.write_byte(0x45).unwrap();
        writer.write_bytes([0x01, 0x02, 0x03, 0x04, 0x05].as_slice()).unwrap();
        writer.write_char('\u{2603}').unwrap();
        writer.write_f64(727.247_f64).unwrap();
        cfg_if::cfg_if!{
            if #[cfg(feature = "f16")] {
                writer.write_f16(247_f16).unwrap();
            } else {
                writer.write_bytes([0x00, 0x00].as_slice()).unwrap();
            }
        }
        writer.write_i16(-5).unwrap();
        writer.write_i32(-100).unwrap();
        writer.write_i64(-2147483649).unwrap();
        writer.write_i8(-112).unwrap();
        writer.write_f32(5.2_f32).unwrap();
        writer.write_7_bit_encoded_int(-723).unwrap();
        writer.write_7_bit_encoded_int(404).unwrap();
        writer.write_7_bit_encoded_int64(9000000000000000000).unwrap();
        writer.write_7_bit_encoded_int64(-500000000000000000).unwrap();
        writer.write_string("meowmeowmeowmeowmeow").unwrap();
        writer.write_u16(624).unwrap();
        writer.write_u32(3000000000).unwrap();
        writer.write_u64(42307830165).unwrap();
        writer.write_char('\0').unwrap();

        cfg_if::cfg_if!{
            if #[cfg(feature = "f16")] {
                cmd!(sh, "dotnet run verify f16").run().unwrap();
            } else {
                cmd!(sh, "dotnet run verify").run().unwrap();
            }
        }
    }

    #[test] 
    fn overflow_7_bit_encoded_int() -> Result<(), DataDecodeError>{
        use std::io::Cursor;
        let data: [u8; 15] = [ 0xFF; 15 ];
        let cursor = Cursor::new(data);

        let mut reader = BinaryReader::new(cursor);
        let result = reader.read_7_bit_encoded_int();
        if let Ok(_inner) = result {
            panic!() // it should have errored
        }

        let result_64 = reader.read_7_bit_encoded_int64();
        if let Ok(_inner) = result_64 {
            panic!() // it should have errored
        }
        Ok(())
    }
}
