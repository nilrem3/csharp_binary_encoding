#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::io::{Error, Read};
use std::error::Error as stdError;
use std::fmt::{Display, Formatter};

/// Indicates that an error has occured because the bytes being decoded were invalid in some way.
#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DataDecodeError{
    /// The underlying reader did not return enough data to construct the type being read.
    NotEnoughBytes,
    /// The underlying data would overflow the current integer type being constructed.
    IntegerOverflow,
    /// The underlaying data cannot be converted to the type because it is not valid utf-8
    InvalidUtf8
}

impl Display for DataDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotEnoughBytes => write!(f, "not enough bytes to decode"),
            Self::IntegerOverflow => write!(f, "decoded integer would overflow"),
            Self::InvalidUtf8 => write!(f, "data cannot be decoded as valid utf8"),
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

/// used like the question mark operator in functions with nested result return types
macro_rules! propogate_inner_error {
    ($nested_error:expr) => {
        match $nested_error {
            Ok(val) => val,
            Err(e) => return Ok(Err(e))
        }
    }
}

/// Analogous to the [`System.IO.BinaryReader`] C# Class.
///
/// Reads from any Read implementor. 
///
///
/// [`System.IO.BinaryReader`]: <https://learn.microsoft.com/en-us/dotnet/api/system.io.binaryreader>
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct BinaryReader<T: Read> {
    input: T,
    buf: Vec<u8>
}

/// All functions in this implementation return an error if the underlying Read returns an error,
/// or if there aren't enough bytes to read.  Individual functions list additional error
/// conditions.
impl<T> BinaryReader<T> 
where T: Read {

    ///Creates a new BinaryReader which will read data from the provided Reader.
    pub fn new(input: T) -> Self {
        Self {
            input,
            buf: Vec::new()
        }
    }

    /// Returns true if enough bytes could be allocated, false otherwise, and Err if the underlying
    /// reader returned an error.
    fn ensure_internal_buffer_size(&mut self, min_size: usize) -> Result<bool, Error>{
        if self.buf.len() >= min_size {
            return Ok(true);
        }

        self.input.read_to_end(&mut self.buf)?;

        Ok(self.buf.len() >= min_size)
    }

    /// Equivalent to the ReadByte method in C#. Reads one byte from the stream. 
    pub fn read_byte(&mut self) -> Result<Result<u8, DataDecodeError>, Error> {
        let val = propogate_inner_error!(self.read_bytes(1)?);
        Ok(Ok(val[0]))
    }

    /// Equivalent to the ReadBytes method in C#. Reads the specified number of bytes.
    pub fn read_bytes(&mut self, num_bytes: usize) -> Result<Result<Vec<u8>, DataDecodeError>, Error> {
        if num_bytes > self.buf.len() && !self.ensure_internal_buffer_size(num_bytes)? {
            return Ok(Err(DataDecodeError::NotEnoughBytes))
        }
        let output = Vec::from_iter(self.buf.drain(0..num_bytes));
        Ok(Ok(output))
    }
    
    /// Doesn't correspond to any specific c# method. Provided for convenience. Gets the next byte
    /// without advancing the data stream.
    pub fn peek_byte(&mut self) -> Result<Result<u8, DataDecodeError>, Error> {
        Ok(Ok(propogate_inner_error!(self.read_bytes(1)?)[0]))
    }
    
    /// Doesn't correspond to any specific c# method. Provided for convenience. Gets the specified
    /// number of bytes without advancing the data stream.
    pub fn peek_bytes(&mut self, num_bytes: usize) -> Result<Result<&[u8], DataDecodeError>, Error> {
        if !self.ensure_internal_buffer_size(num_bytes)? {
            Ok(Err(DataDecodeError::NotEnoughBytes))
        } else {
            Ok(Ok(&self.buf.as_slice()[1..num_bytes]))
        }
    }
    
    /// Equivalent to the Read7BitEncodedInt method in C#.
    /// Returns [DataDecodeError::IntegerOverflow] if the encoded value does not fit within 32 bits.
    pub fn read_7_bit_encoded_int(&mut self) -> Result<Result<i32, DataDecodeError>, Error> {
        const MAX_BYTES: u32 = 5;
        let mut output: i32 = 0;
        let mut bytes_read = 0;
        loop {
            let byte =  propogate_inner_error!(self.read_byte()?);
            let lower_bits = byte & 0b01111111;
            let high_bit = byte & 0b10000000;
            output += (lower_bits as i32) << (7 * bytes_read);
            if high_bit == 0 {
                return Ok(Ok(output))
            } 
            bytes_read+=1;
            if bytes_read >= MAX_BYTES - 1{
                break; // need to handle the most significant bit specially
            }
        }
        
        let max_value_for_most_significant_bit = u8::pow(2, 32 - 28) - 1;
        let last_byte: u8 = propogate_inner_error!(self.read_byte()?);
        if last_byte > max_value_for_most_significant_bit {
            Ok(Err(DataDecodeError::IntegerOverflow))
        } else {
            Ok(Ok(output + ((last_byte as i32) << 28_i32)))
        }
    }
    
    /// Equivalent to the Read7BitEncodedInt64 method in C#.
    /// Returns [DataDecodeError::IntegerOverflow] if the encoded value does not fit within 64 bits.
    pub fn read_7_bit_encoded_int64(&mut self) -> Result<Result<i64, DataDecodeError>, Error> {
        const MAX_BYTES: u32 = 10;
        let mut output: i64 = 0; 
        let mut bytes_read = 0;
        loop {
            let byte: u8 = propogate_inner_error!(self.read_byte()?);
            let lower_bits = byte & 0b01111111;
            let high_bit = byte & 0b10000000;
            output += (lower_bits as i64) << (7 * bytes_read);
            if high_bit == 0 {
                return Ok(Ok(output));
            }
            bytes_read+=1;
            if bytes_read >= MAX_BYTES - 1 {
                break;
            }
        }

        let max_value_for_most_significant_bit = u8::pow(2, 64 - 63) - 1;
        let last_byte = propogate_inner_error!(self.read_byte()?);
        if last_byte > max_value_for_most_significant_bit {
            Ok(Err(DataDecodeError::IntegerOverflow))
        } else {
            Ok(Ok(output + ((last_byte as i64) << 63)))
        }
    }
    
    /// Equivalent to the ReadBoolean method in C#.
    pub fn read_boolean(&mut self) -> Result<Result<bool, DataDecodeError>, Error> {
        let byte = propogate_inner_error!(self.read_byte()?);
        Ok(Ok(byte != 0))
    }
    
    /// Equivalent to the ReadSingle method in C#.
    pub fn read_f32(&mut self) -> Result<Result<f32, DataDecodeError>, Error> {
        let bytes: [u8; 4] = propogate_inner_error!(self.read_bytes(4)?).try_into().unwrap();
        Ok(Ok(f32::from_le_bytes(bytes)))
    }

    /// Equivalent to the ReadDouble method in C#.
    pub fn read_f64(&mut self) -> Result<Result<f64, DataDecodeError>, Error> {
        let bytes: [u8; 8] = propogate_inner_error!(self.read_bytes(8)?).try_into().unwrap();
        Ok(Ok(f64::from_le_bytes(bytes)))
    }
    
    /// Equivalent to the ReadHalf method in C#.
    /// Requires the `f16` feature.
    #[cfg_attr(docsrs, doc(cfg(feature = "f16")))]
    #[cfg(feature = "f16")]
    pub fn read_f16(&mut self) -> Result<Result<f16, DataDecodeError>, Error> {
        let bytes: [u8; 2] = propogate_inner_error!(self.read_bytes(2)?).try_into().unwrap();
        Ok(Ok(f16::from_le_bytes(bytes)))
    }
    
    /// Equivalent to the ReadString method in C#.
    /// Returns an [DataDecodeError::InvalidUtf8] if the data read is not valid utf-8.
    pub fn read_string(&mut self) -> Result<Result<String, DataDecodeError>, Error> {
        let length: usize = propogate_inner_error!(self.read_7_bit_encoded_int()?) as usize;
        let string_bytes = propogate_inner_error!(self.read_bytes(length)?);
        match std::str::from_utf8(string_bytes.as_slice()) {
            Ok(v) => Ok(Ok(v.to_string())),
            Err(_) => Ok(Err(DataDecodeError::InvalidUtf8))
        }
    }
    
    /// Equivalent to the ReadSByte method in C#.
    pub fn read_i8(&mut self) -> Result<Result<i8, DataDecodeError>, Error> {
        let bytes: [u8; 1] = [propogate_inner_error!(self.read_byte()?)];
        Ok(Ok(i8::from_le_bytes(bytes)))
    }

    /// Equivalent to the ReadInt16 method in C#.
    pub fn read_i16(&mut self) -> Result<Result<i16, DataDecodeError>, Error> {
        let bytes: [u8; 2] = propogate_inner_error!(self.read_bytes(2)?).try_into().unwrap();
        Ok(Ok(i16::from_le_bytes(bytes)))
    }

    /// Equivalent to the ReadInt32 method in C#.
    pub fn read_i32(&mut self) -> Result<Result<i32, DataDecodeError>, Error> {
        let bytes: [u8; 4] = propogate_inner_error!(self.read_bytes(4)?).try_into().unwrap();
        Ok(Ok(i32::from_le_bytes(bytes)))
    }

    /// Equivalent to the ReadInt64 method in C#.
    pub fn read_i64(&mut self) -> Result<Result<i64, DataDecodeError>, Error> {
        let bytes: [u8; 8] = propogate_inner_error!(self.read_bytes(8)?).try_into().unwrap();
        Ok(Ok(i64::from_le_bytes(bytes)))
    }

    /// Equivalent to the ReadUint16 method in C#.
    pub fn read_u16(&mut self) -> Result<Result<u16, DataDecodeError>, Error> {
        let bytes: [u8; 2] = propogate_inner_error!(self.read_bytes(2)?).try_into().unwrap();
        Ok(Ok(u16::from_le_bytes(bytes)))
    }

    /// Equivalent to the ReadUint32 method in C#.
    pub fn read_u32(&mut self) -> Result<Result<u32, DataDecodeError>, Error> {
        let bytes: [u8; 4] = propogate_inner_error!(self.read_bytes(4)?).try_into().unwrap();
        Ok(Ok(u32::from_le_bytes(bytes)))
    }

    /// Equivalent to the ReadUint64 method in C#.
    pub fn read_u64(&mut self) -> Result<Result<u64, DataDecodeError>, Error> {
        let bytes: [u8; 8] = propogate_inner_error!(self.read_bytes(8)?).try_into().unwrap();
        Ok(Ok(u64::from_le_bytes(bytes)))
    }
    
    // Implementation translated from the c# dotnet runtime's implementation of BinaryReader
    // MIT Licensed by the .NET foundation, can be found at https://github.com/dotnet/runtime
    // Returns [DataDecodeError::InvalidUtf8] if the next character is not a valid character in
    // utf-8
    pub fn read_char(&mut self) -> Result<Result<char, DataDecodeError>, Error> {
        const MAX_BYTES_PER_CHAR: usize = 4;
        let mut bytes: [u8; MAX_BYTES_PER_CHAR] = [0; MAX_BYTES_PER_CHAR];
        let mut current_index: usize = 0;
        let mut num_chars_read: usize = 0;
        let mut decode_result: Result<String, std::string::FromUtf8Error>;
        loop { 
            bytes[current_index] = propogate_inner_error!(self.read_byte()?);
            decode_result = String::from_utf8(bytes.to_vec());
            if let Ok(result) = &decode_result {
                let result = result.trim_matches(char::from(0)); // trim null bytes
                num_chars_read = result.chars().count();
                break;
            } else {
                current_index+=1;
                if current_index >= MAX_BYTES_PER_CHAR {
                    break;
                }
            }
        }
        if num_chars_read == 1 {
            if let Ok(result) = decode_result {
                return Ok(Ok(result.trim_matches(char::from(0)).chars().next().expect("?")))
            } 
        } 
        Ok(Err(DataDecodeError::InvalidUtf8)) // read two chars somehow
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

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
