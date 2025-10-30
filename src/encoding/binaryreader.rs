use std::io::{Read};
use std::error::Error as stdError;
use thiserror::Error;
use std::fmt::{Display, Formatter};

/// Indicates that an error occured while decoding the data.
#[derive(Error, Debug)]
pub enum DataDecodeError {
    /// An error occured while trying to read the data.
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// The value of the data itself led to an error.
    #[error(transparent)]
    InvalidData(#[from] InvalidDataError)
}

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

/// Analogous to the [`System.IO.BinaryReader`] C# Class.
///
/// Reads from any Read implementor. 
///
///
/// [`System.IO.BinaryReader`]: <https://learn.microsoft.com/en-us/dotnet/api/system.io.binaryreader>
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct BinaryReader<T: Read> {
    input: T,
    buf: Vec<u8>,
    num_bytes_read: u64
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
            buf: Vec::new(),
            num_bytes_read: 0
        }
    }

    /// Returns the total number of bytes that have been read from the input Reader so far.
    pub fn num_bytes_read(&self) -> u64 {
        self.num_bytes_read
    }

    /// Returns true if enough bytes could be allocated, false otherwise, and Err if the underlying
    /// reader returned an error.
    fn ensure_internal_buffer_size(&mut self, min_size: usize) -> Result<bool, std::io::Error>{
        if self.buf.len() >= min_size {
            return Ok(true);
        }

        self.input.read_to_end(&mut self.buf)?;

        Ok(self.buf.len() >= min_size)
    }

    /// Equivalent to the ReadByte method in C#. Reads one byte from the stream. 
    pub fn read_byte(&mut self) -> Result<u8, DataDecodeError> {
        Ok(self.read_bytes(1)?[0])
    }

    /// Equivalent to the ReadBytes method in C#. Reads the specified number of bytes.
    pub fn read_bytes(&mut self, num_bytes: usize) -> Result<Vec<u8>, DataDecodeError> {
        if num_bytes > self.buf.len() && !self.ensure_internal_buffer_size(num_bytes)? {
            return Err(DataDecodeError::InvalidData(InvalidDataError::NotEnoughBytes))
        }
        self.num_bytes_read += num_bytes as u64;
        Ok(Vec::from_iter(self.buf.drain(0..num_bytes)))
    }
    
    /// Doesn't correspond to any specific c# method. Provided for convenience. Gets the next byte
    /// without advancing the data stream.
    pub fn peek_byte(&mut self) -> Result<u8, DataDecodeError> {
        Ok(self.peek_bytes(1)?[0])
    }
    
    /// Doesn't correspond to any specific c# method. Provided for convenience. Gets the specified
    /// number of bytes without advancing the data stream.
    pub fn peek_bytes(&mut self, num_bytes: usize) -> Result<&[u8], DataDecodeError> {
        if !self.ensure_internal_buffer_size(num_bytes)? {
            Err(DataDecodeError::InvalidData(InvalidDataError::NotEnoughBytes))
        } else {
            Ok(&self.buf.as_slice()[0..num_bytes])
        }
    }
    
    /// Equivalent to the Read7BitEncodedInt method in C#.
    /// Returns [DataDecodeError]::InvalidData([InvalidDataError::IntegerOverflow]) if the encoded value does not fit within 32 bits.
    /// if the integer overflows, the bytes will still be consumed.
    pub fn read_7_bit_encoded_int(&mut self) -> Result<i32, DataDecodeError> {
        const MAX_BYTES: u32 = 5;
        let mut output: i32 = 0;
        let mut bytes_read = 0;
        loop {
            let byte =  self.read_byte()?;
            let lower_bits = byte & 0b01111111;
            let high_bit = byte & 0b10000000;
            output += (lower_bits as i32) << (7 * bytes_read);
            if high_bit == 0 {
                return Ok(output)
            } 
            bytes_read+=1;
            if bytes_read >= MAX_BYTES - 1{
                break; // need to handle the most significant bit specially
            }
        }
        
        let max_value_for_most_significant_bit = u8::pow(2, 32 - 28) - 1;
        let last_byte: u8 = self.read_byte()?;
        if last_byte > max_value_for_most_significant_bit {
            Err(DataDecodeError::InvalidData(InvalidDataError::IntegerOverflow))
        } else {
            Ok(output + ((last_byte as i32) << 28_i32))
        }
    }
    
    /// Equivalent to the Read7BitEncodedInt64 method in C#.
    /// Returns [DataDecodeError]::InvalidData([InvalidDataError::IntegerOverflow]) if the encoded value does not fit within 64 bits.
    /// if the integer overflows, the bytes will still be consumed
    pub fn read_7_bit_encoded_int64(&mut self) -> Result<i64, DataDecodeError> {
        const MAX_BYTES: u32 = 10;
        let mut output: i64 = 0; 
        let mut bytes_read = 0;
        loop {
            let byte: u8 = self.read_byte()?;
            let lower_bits = byte & 0b01111111;
            let high_bit = byte & 0b10000000;
            output += (lower_bits as i64) << (7 * bytes_read);
            if high_bit == 0 {
                return Ok(output);
            }
            bytes_read+=1;
            if bytes_read >= MAX_BYTES - 1 {
                break;
            }
        }

        let max_value_for_most_significant_bit = u8::pow(2, 64 - 63) - 1;
        let last_byte = self.read_byte()?;
        if last_byte > max_value_for_most_significant_bit {
            Err(InvalidDataError::IntegerOverflow.into())
        } else {
            Ok(output + ((last_byte as i64) << 63))
        }
    }
    
    /// Equivalent to the ReadBoolean method in C#.
    pub fn read_boolean(&mut self) -> Result<bool, DataDecodeError> {
        let byte = self.read_byte()?;
        Ok(byte != 0)
    }
    
    /// Equivalent to the ReadSingle method in C#.
    pub fn read_f32(&mut self) -> Result<f32, DataDecodeError> {
        let bytes: [u8; 4] = self.read_bytes(4)?.try_into().unwrap();
        Ok(f32::from_le_bytes(bytes))
    }

    /// Equivalent to the ReadDouble method in C#.
    pub fn read_f64(&mut self) -> Result<f64, DataDecodeError> {
        let bytes: [u8; 8] = self.read_bytes(8)?.try_into().unwrap();
        Ok(f64::from_le_bytes(bytes))
    }
    
    /// Equivalent to the ReadHalf method in C#.
    /// Requires the `f16` feature.
    #[cfg_attr(docsrs, doc(cfg(feature = "f16")))]
    #[cfg(feature = "f16")]
    pub fn read_f16(&mut self) -> Result<f16, DataDecodeError> {
        let bytes: [u8; 2] = self.read_bytes(2)?.try_into().unwrap();
        Ok(f16::from_le_bytes(bytes))
    }
    
    /// Equivalent to the ReadString method in C#.
    /// Returns an [DataDecodeError]::InvalidData([InvalidDataError::InvalidUtf8]) if the data read is not valid utf-8.
    /// This function can consume some bytes even when it fails.
    pub fn read_string(&mut self) -> Result<String, DataDecodeError> {
        let length: usize = self.read_7_bit_encoded_int()?.try_into().unwrap();
        let string_bytes = self.read_bytes(length)?;
        match std::str::from_utf8(string_bytes.as_slice()) {
            Ok(v) => Ok(v.to_string()),
            Err(_) => Err(DataDecodeError::InvalidData(InvalidDataError::InvalidUtf8))
        }
    }
    
    /// Equivalent to the ReadSByte method in C#.
    pub fn read_i8(&mut self) -> Result<i8, DataDecodeError> {
        let bytes: [u8; 1] = [self.read_byte()?];
        Ok(i8::from_le_bytes(bytes))
    }

    /// Equivalent to the ReadInt16 method in C#.
    pub fn read_i16(&mut self) -> Result<i16, DataDecodeError> {
        let bytes: [u8; 2] = self.read_bytes(2)?.try_into().unwrap();
        Ok(i16::from_le_bytes(bytes))
    }

    /// Equivalent to the ReadInt32 method in C#.
    pub fn read_i32(&mut self) -> Result<i32, DataDecodeError> {
        let bytes: [u8; 4] = self.read_bytes(4)?.try_into().unwrap();
        Ok(i32::from_le_bytes(bytes))
    }

    /// Equivalent to the ReadInt64 method in C#.
    pub fn read_i64(&mut self) -> Result<i64, DataDecodeError> {
        let bytes: [u8; 8] = self.read_bytes(8)?.try_into().unwrap();
        Ok(i64::from_le_bytes(bytes))
    }

    /// Equivalent to the ReadUint16 method in C#.
    pub fn read_u16(&mut self) -> Result<u16, DataDecodeError> {
        let bytes: [u8; 2] = self.read_bytes(2)?.try_into().unwrap();
        Ok(u16::from_le_bytes(bytes))
    }

    /// Equivalent to the ReadUint32 method in C#.
    pub fn read_u32(&mut self) -> Result<u32, DataDecodeError> {
        let bytes: [u8; 4] = self.read_bytes(4)?.try_into().unwrap();
        Ok(u32::from_le_bytes(bytes))
    }

    /// Equivalent to the ReadUint64 method in C#.
    pub fn read_u64(&mut self) -> Result<u64, DataDecodeError> {
        let bytes: [u8; 8] = self.read_bytes(8)?.try_into().unwrap();
        Ok(u64::from_le_bytes(bytes))
    }
    
    // Implementation translated from the c# dotnet runtime's implementation of BinaryReader
    // MIT Licensed by the .NET foundation, can be found at https://github.com/dotnet/runtime
    /// Equivalent to the ReadChar method in C#.
    /// Returns [DataDecodeError]::InvalidData([InvalidDataError::InvalidUtf8]) if the next character is not a valid character in
    /// utf-8
    /// this function can consume some bytes even when it fails.
    pub fn read_char(&mut self) -> Result<char, DataDecodeError> {
        const MAX_BYTES_PER_CHAR: usize = 4;
        let mut bytes: [u8; MAX_BYTES_PER_CHAR] = [0; MAX_BYTES_PER_CHAR];
        let mut current_index: usize = 0;
        let mut num_chars_read: usize = 0;
        let mut decode_result: Result<String, std::string::FromUtf8Error>;
        loop { 
            bytes[current_index] = self.read_byte()?;
            decode_result = String::from_utf8(bytes.to_vec());
            if let Ok(result) = &decode_result {
                let mut result = result.as_str();
                // trim null bytes, but always keep at least one byte
                while result.chars().last() == Some(char::from(0)) && result.chars().collect::<Vec<_>>().len() > 1 {
                    result = &result[0..result.len() - 1]; 
                }
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
                return Ok(result.chars().next().expect("?"))
            } 
        } 
        Err(DataDecodeError::InvalidData(InvalidDataError::InvalidUtf8)) // read two chars somehow
    }
    
}
