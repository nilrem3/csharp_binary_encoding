use std::io::{prelude::*, Error, ErrorKind};

pub struct BinaryReader<T: Read> {
    input: T,
    buf: Vec<u8>
}



impl<T> BinaryReader<T> 
where T: Read {
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

        loop {
            let byte: u8 = self.read_byte()?;
            let lower_bits = byte & 0b01111111;
            let high_bit = byte & 0b10000000;
            output += lower_bits as i32;
            if high_bit == 0 {
                break;
            } 
            output <<= 7;
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
            if let Ok(result) = decode_result {
                num_chars_read = result.len();
            } else {
                current_index+=1;
                if current_index >= MAX_BYTES_PER_CHAR {
                    break;
                }
            }
        }
        if num_chars_read == 1 {
            return Ok(decode_result.expect("Failed to decode character.").chars().next().expect("should be impossible"));
        }
        Err(Error::new(ErrorKind::InvalidData, "Failed to read exactly one character"))
    }
    
}
