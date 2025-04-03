#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::error::Error as stdError;
use std::fmt::{Display, Formatter};

#[macro_use]
mod macros{
    /// used like the question mark operator in functions with nested result return types
    macro_rules! propogate_inner_error {
        ($nested_error:expr) => {
            match $nested_error {
                Ok(val) => val,
                Err(e) => return Ok(Err(e))
            }
        }
    }
}

mod encoding {
    mod binaryreader;
    pub use binaryreader::BinaryReader;
    mod binarywriter;
    pub use binarywriter::BinaryWriter;
}
pub use encoding::BinaryReader;
pub use encoding::BinaryWriter;


/// Indicates that an error has occured because the bytes being decoded were invalid in some way.
/// Note: In versions 0.2.0 and before this was called DataDecodeError.
#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum InvalidDataError{
    /// The underlying reader did not return enough data to construct the type being read.
    NotEnoughBytes,
    /// The underlying data overflowed the current integer type being constructed.
    IntegerOverflow,
    /// The underlaying data could not be converted to the type because it is not valid utf-8
    InvalidUtf8
}

impl Display for InvalidDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotEnoughBytes => write!(f, "not enough bytes to decode"),
            Self::IntegerOverflow => write!(f, "decoded integer overflowed"),
            Self::InvalidUtf8 => write!(f, "data could not be decoded as valid utf8"),
        }
    }
}

impl stdError for InvalidDataError{
    fn source(&self) -> Option<&(dyn stdError + 'static)> {
        None // there isn't a lower-level error source
    }

    fn description(&self) -> &str {
        "use of deprecated description() method on std::error::Error"
    }

    fn cause(&self) -> Option<&dyn stdError> {
        None // deprecated
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    const TEST_FOLDER: &str = "csharp_testing";

    #[test]
    fn decode_all_types() -> Result<Result<(), InvalidDataError>, Error>{
        use std::fs::File;
        use xshell::{Shell, cmd};
        let sh = Shell::new().unwrap();
        sh.change_dir(&TEST_FOLDER.to_string());
        cmd!(sh, "dotnet run generate").run().unwrap();

        let file = File::open(TEST_FOLDER.to_string()+ "/output.bin")?;
        let mut reader = BinaryReader::new(file);

        // read the test data written in generate_test_bin.cs
        assert!(propogate_inner_error!(reader.read_boolean()?));
        assert!(!propogate_inner_error!(reader.read_boolean()?));
        assert_eq!(0x45, propogate_inner_error!(reader.read_byte()?));
        assert_eq!(vec![0x01, 0x02, 0x03, 0x04, 0x05], propogate_inner_error!(reader.read_bytes(5)?));
        assert_eq!('\u{2603}' as char, propogate_inner_error!(reader.read_char()?));
        assert_eq!(727.247_f64, propogate_inner_error!(reader.read_f64()?));
        cfg_if::cfg_if! {
            if #[cfg(feature = "f16")] {
                assert_eq!(247_f16, propogate_inner_error!(reader.read_f16()?));
            } else {
                propogate_inner_error!(reader.read_bytes(2)?); // just skip the two bytes instead
            }
        }
        assert_eq!(-5_i16, propogate_inner_error!(reader.read_i16()?));
        assert_eq!(-100_i32, propogate_inner_error!(reader.read_i32()?));
        assert_eq!(-2147483649_i64, propogate_inner_error!(reader.read_i64()?));
        assert_eq!(-112_i8, propogate_inner_error!(reader.read_i8()?));
        assert_eq!(5.2_f32, propogate_inner_error!(reader.read_f32()?));
        assert_eq!(-723_i32, propogate_inner_error!(reader.read_7_bit_encoded_int()?));
        assert_eq!(404_i32, propogate_inner_error!(reader.read_7_bit_encoded_int()?));
        assert_eq!(9000000000000000000_i64, propogate_inner_error!(reader.read_7_bit_encoded_int64()?));
        assert_eq!(-500000000000000000_i64, propogate_inner_error!(reader.read_7_bit_encoded_int64()?));
        assert_eq!("meowmeowmeowmeowmeow".to_string(), propogate_inner_error!(reader.read_string()?));
        assert_eq!(624_u16, propogate_inner_error!(reader.read_u16()?));
        assert_eq!(3000000000_u32, propogate_inner_error!(reader.read_u32()?));
        assert_eq!(42307830165_u64, propogate_inner_error!(reader.read_u64()?));

        let _ = cmd!(sh, "rm -f output.bin").run();

        Ok(Ok(()))
    }

    #[test]
    fn encode_all_types(){
        use std::fs::File;
        use xshell::{Shell, cmd};
        
        let sh = Shell::new().unwrap();
        sh.change_dir(&TEST_FOLDER.to_string());

        let _ = cmd!(sh, "rm -f input.bin").run().unwrap();

        let file = File::create(TEST_FOLDER.to_string() + "/input.bin").unwrap();
        let mut writer = BinaryWriter::new(file);

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

        cfg_if::cfg_if!{
            if #[cfg(feature = "f16")] {
                cmd!(sh, "dotnet run verify f16").run().unwrap();
            } else {
                cmd!(sh, "dotnet run verify").run().unwrap();
            }
        }
    }

    #[test] 
    fn overflow_7_bit_encoded_int() -> Result<Result<(), InvalidDataError>, Error>{
        use std::io::Cursor;
        let data: [u8; 15] = [ 0xFF; 15 ];
        let cursor = Cursor::new(data);

        let mut reader = BinaryReader::new(cursor);
        let result = reader.read_7_bit_encoded_int();
        if let Ok(inner) = result {
            if let Ok(_) = inner {
                panic!() // it should have errored
            }
        }

        let result_64 = reader.read_7_bit_encoded_int64();
        if let Ok(inner) = result_64 {
            if let Ok(_) = inner {
                panic!() // it should have errored
            }
            
        }
        Ok(Ok(()))
    }
}
