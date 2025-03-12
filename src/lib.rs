use std::io::{prelude::*, Error, ErrorKind};

pub struct BinaryReader<T: Read> {
    input: T,
    buf: Vec<u8>
}



impl<T> BinaryReader<T> 
where T: Read {
    pub fn new(input: T) -> Self {
        Self {
            input,
            buf: Vec::new()
        }
    }

    fn ensure_internal_buffer_size(self: &mut Self, min_size: usize) -> Result<(), Error>{
        if self.buf.len() >= min_size {
            return Ok(());
        }

        let error = self.input.read_to_end(&mut self.buf);
        if let Err(x) = error {
            return Err(x);
        }

        if self.buf.len() < min_size {
            return Err(Error::new(ErrorKind::UnexpectedEof, "Not enough bytes avaialable to read"));
        }

        return Ok(());
    }

    pub fn read_bytes(self: &mut Self, num_bytes: usize) -> Result<Vec<u8>, Error> {
        if num_bytes > self.buf.len() {
            let enough_bytes_available = self.ensure_internal_buffer_size(num_bytes);
            if let Err(e) = enough_bytes_available {
                return Err(e);
            }
        }
        let output = Vec::from_iter(self.buf.drain(0..num_bytes));
        return Ok(output);
    }

    pub fn peek_bytes(self: &mut Self, num_bytes: usize) -> Result<&[u8], Error> {
        if let Err(e) = self.ensure_internal_buffer_size(num_bytes) {
            return Err(e);
        }
        return Ok(&self.buf.as_slice()[1..num_bytes]);
    }

    pub fn peek_byte(self: &mut Self) -> Result<u8, Error> {
        if let Err(e) = self.ensure_internal_buffer_size(1) {
            return Err(e);
        }
        return Ok(self.buf[0]);
    }

    pub fn read_byte(self: &mut Self) -> Result<u8, Error> {
        let vec = self.read_bytes(1)?;
        return Ok(vec[0]);
    }

    pub fn read_7_bit_encoded_int(self: &mut Self) -> Result<i32, Error> {
        let mut output: i32 = 0;
        let mut bytes_read = 0;
        loop {
            let byte: u8 = self.read_byte()?;
            let lower_bits = byte & 0b01111111;
            let high_bit = byte & 0b10000000;
            output += (lower_bits as i32) << (7 * bytes_read);
            if high_bit == 0 {
                break;
            } 
            bytes_read+=1;
        }
        
        return Ok(output);
    }

    pub fn read_7_bit_encoded_int64(self: &mut Self) -> Result<i64, Error> {
        let mut output: i64 = 0; 
        let mut bytes_read = 0;
        loop {
            let byte: u8 = self.read_byte()?;
            let lower_bits = byte & 0b01111111;
            let high_bit = byte & 0b10000000;
            output += (lower_bits as i64) << (7 * bytes_read);
            if high_bit == 0 {
                break;
            }
            bytes_read+=1;
        }

        return Ok(output);
    }

    pub fn read_boolean(self: &mut Self) -> Result<bool, Error> {
        let byte = self.read_byte()?;
        return Ok(byte != 0);
    }

    pub fn read_single(self: &mut Self) -> Result<f32, Error> {
        let bytes: [u8; 4] = self.read_bytes(4)?.try_into().unwrap();
        return Ok(f32::from_le_bytes(bytes));
    }

    pub fn read_double(self: &mut Self) -> Result<f64, Error> {
        let bytes: [u8; 8] = self.read_bytes(8)?.try_into().unwrap();
        return Ok(f64::from_le_bytes(bytes));
    }

    #[cfg(feature = "f16_support")]
    pub fn read_half(self: &mut Self) -> Result<f16, Error> {
        let bytes: [u8; 2] = self.read_bytes(2)?.try_into().unwrap();
        return Ok(f16::from_le_bytes(bytes));
    }

    pub fn read_string(self: &mut Self) -> Result<String, Error> {
        let length: usize = self.read_7_bit_encoded_int()? as usize;
        let string_bytes = self.read_bytes(length)?;
        match std::str::from_utf8(string_bytes.as_slice()) {
            Ok(v) => Ok(v.to_string()),
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "Utf8 Error"))
        }
    }

    pub fn read_i16(self: &mut Self) -> Result<i16, Error> {
        let bytes: [u8; 2] = self.read_bytes(2)?.try_into().unwrap();
        return Ok(i16::from_le_bytes(bytes));
    }

    pub fn read_i32(self: &mut Self) -> Result<i32, Error> {
        let bytes: [u8; 4] = self.read_bytes(4)?.try_into().unwrap();
        return Ok(i32::from_le_bytes(bytes));
    }

    pub fn read_i64(self: &mut Self) -> Result<i64, Error> {
        let bytes: [u8; 8] = self.read_bytes(8)?.try_into().unwrap();
        return Ok(i64::from_le_bytes(bytes));
    }
    
    // Implementation taken from the c# dotnet runtime's implementation of BinaryReader
    pub fn read_char(self: &mut Self) -> Result<char, Error> {
        const MAX_BYTES_PER_CHAR: usize = 4;
        let mut bytes: [u8; MAX_BYTES_PER_CHAR] = [0; MAX_BYTES_PER_CHAR];
        let mut current_index: usize = 0;
        let mut num_chars_read: usize = 0;
        let mut decode_result: Result<String, std::string::FromUtf8Error>;
        loop { 
            bytes[current_index] = self.read_byte()?;
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
                return Ok(result.trim_matches(char::from(0)).chars().next().expect("?"))
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Failed to decode bytes"));
            }
        }
        Err(Error::new(ErrorKind::InvalidData, "Failed to read exactly one character"))
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn decode_all_types() -> Result<(), Error>{
        let file = File::open("cs_test_input_generation/output.bin")?;
        let mut reader = BinaryReader::new(file);
        // read the test data written in Program.cs
        assert!(reader.read_boolean()?);
        assert!(!reader.read_boolean()?);
        assert_eq!(0x45, reader.read_byte()?);
        assert_eq!(vec![0x01, 0x02, 0x03, 0x04, 0x05], reader.read_bytes(5)?);
        assert_eq!('\u{2603}' as char, reader.read_char()?);
        //TODO: properly test reading of f16 
        //assert_eq!(727.247_f64, reader.read_double()?);
        reader.read_bytes(2)?; // just skip the two bits instead
        assert_eq!(247_f16, reader.read_half()?);
        assert_eq!(-5_i16, reader.read_i16()?);
        assert_eq!(-100_i32, reader.read_i32()?);
        assert_eq!(-2147483649_i64, reader.read_i64()?);
        //TODO: read i8
        //assert_eq!(-112_i8, reader.read_i8()?);
        reader.read_byte()?; // deal with i8 byte we don't have a method to read yet
        assert_eq!(5.2_f32, reader.read_single()?);
        assert_eq!("meowmeowmeowmeowmeow".to_string(), reader.read_string()?);
        //TODO: read u16
        //assert_eq!(624_u16, reader.read_u16());
        reader.read_bytes(2)?; // deal with bytes from u16
        //TODO: read u32
        //assert_eq!(3000000000_u32, reader.read_u32());
        reader.read_bytes(4)?; // deal with bytes from u32;
        //TODO: read u64
        //assert_eq!(42307830165_u64, reader.read_u64());
        reader.read_bytes(8)?; // deal with bytes from u64;
        assert_eq!(-723_i32, reader.read_7_bit_encoded_int()?);
        assert_eq!(404_i32, reader.read_7_bit_encoded_int()?);
        assert_eq!(9000000000000000000_i64, reader.read_7_bit_encoded_int64()?);
        assert_eq!(-500000000000000000_i64, reader.read_7_bit_encoded_int64()?);
        Ok(())
    }
}
