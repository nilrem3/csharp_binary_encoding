
use std::io;
use std::io::Write;
/// Analagous to the [`System.IO.BinaryWriter`] C# Class.
///
/// Writes to any Write implementor.
///
/// [`System.IO.BinaryWriter`]:
/// <https://learn.microsoft.com/en-us/dotnet/api/system.io.binarywriter>
pub struct BinaryWriter<T: Write> {
    output: T
}

impl<T> BinaryWriter<T>
where T: Write {
    
    ///Creates a new BinaryWriter which will write data to the provided Writer
    pub fn new(output: T) -> Self {
        Self {
            output
        }
    }

    pub fn write_byte(&mut self, data: u8) -> io::Result<usize> {
        self.output.write(&[data])
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> io::Result<usize> {
        self.output.write(data)
    }

    pub fn write_7_bit_encoded_int(&mut self, data: i32) -> io::Result<usize> {
        unimplemented!()
    }

    pub fn write_7_bit_encoded_int64(&mut self, data: i64) -> io::Result<usize> {
        unimplemented!()
    }

    pub fn write_boolean(&mut self, data: bool) -> io::Result<usize> {
        // explicitely use C#'s binary representation of bool
        // without making assumptions about how rust stores bool values 
        // in memory
        if data {
            self.write_byte(1) 
        } else {
            self.write_byte(0)
        }
    }
    
    pub fn write_f32(&mut self, data: f32) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    pub fn write_f64(&mut self, data: f64) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "f16")))]
    #[cfg(feature = "f16")]
    pub fn write_f16(&mut self, data: f16) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    pub fn write_string(&mut self, data: &str) -> io::Result<usize> {
        unimplemented!()
    }

    pub fn write_i8(&mut self, data: i8) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    pub fn write_i16(&mut self, data: i16) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    pub fn write_i32(&mut self, data: i32) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    pub fn write_i64(&mut self, data: i64) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    pub fn write_u16(&mut self, data: u16) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    pub fn write_u32(&mut self, data: u32) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    pub fn write_u64(&mut self, data: u64) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    pub fn write_char(&mut self, data: char) -> io::Result<usize> {
        unimplemented!()
    }

}
