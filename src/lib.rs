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
}
pub use encoding::BinaryReader;


/// Indicates that an error has occured because the bytes being decoded were invalid in some way.
#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DataDecodeError{
    /// The underlying reader did not return enough data to construct the type being read.
    NotEnoughBytes,
    /// The underlying data overflowed the current integer type being constructed.
    IntegerOverflow,
    /// The underlaying data could not be converted to the type because it is not valid utf-8
    InvalidUtf8
}

impl Display for DataDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotEnoughBytes => write!(f, "not enough bytes to decode"),
            Self::IntegerOverflow => write!(f, "decoded integer overflowed"),
            Self::InvalidUtf8 => write!(f, "data could not be decoded as valid utf8"),
        }
    }
}

impl stdError for DataDecodeError{
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

    #[test]
    fn decode_all_types() -> Result<Result<(), DataDecodeError>, Error>{
        use std::fs::File;
        use xshell::{Shell, cmd};
        //TODO: also run the c# program to generate output.bin from within this test
        let sh = Shell::new().unwrap();
        let folder = "cs_test_input_generation";
        sh.change_dir(folder);
        cmd!(sh, "dotnet run").run().unwrap();

        let file = File::open("cs_test_input_generation/output.bin")?;
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
        assert_eq!("meowmeowmeowmeowmeow".to_string(), propogate_inner_error!(reader.read_string()?));
        assert_eq!(624_u16, propogate_inner_error!(reader.read_u16()?));
        assert_eq!(3000000000_u32, propogate_inner_error!(reader.read_u32()?));
        assert_eq!(42307830165_u64, propogate_inner_error!(reader.read_u64()?));
        assert_eq!(-723_i32, propogate_inner_error!(reader.read_7_bit_encoded_int()?));
        assert_eq!(404_i32, propogate_inner_error!(reader.read_7_bit_encoded_int()?));
        assert_eq!(9000000000000000000_i64, propogate_inner_error!(reader.read_7_bit_encoded_int64()?));
        assert_eq!(-500000000000000000_i64, propogate_inner_error!(reader.read_7_bit_encoded_int64()?));
        Ok(Ok(()))
    }

    #[test] 
    fn overflow_7_bit_encoded_int() -> Result<Result<(), DataDecodeError>, Error>{
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
