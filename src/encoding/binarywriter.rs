
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
    
    /// Equivalent to the Write method in C# called with an argument of type Byte.
    pub fn write_byte(&mut self, data: u8) -> io::Result<usize> {
        self.output.write(&[data])
    }

    /// Equivalent to the Write method in C# called with an argument of type Byte[].
    pub fn write_bytes(&mut self, data: &[u8]) -> io::Result<usize> {
        self.output.write(data)
    }

    // Implementation translated from the c# dotnet runtime's implementation of BinaryWriter
    // MIT Licensed by the .NET foundation, can be found at https://github.com/dotnet/runtime
    /// Equivalent to the Write7BitEncodedInt method in C#.
    pub fn write_7_bit_encoded_int(&mut self, data: i32) -> io::Result<usize> {
        let mut value = data as u32;
        let mut out_bytes: Vec<u8> = Vec::new();
        while value > 0x7F {
            let low_bits_and_flag: u8 = (value | !0x7F).to_le_bytes()[0];
            value >>= 7;
            out_bytes.push(low_bits_and_flag);
        }
        out_bytes.push(value.to_le_bytes()[0]);
        self.write_bytes(&out_bytes)
    }

    // Implementation translated from the c# dotnet runtime's implementation of BinaryWriter
    // MIT Licensed by the .NET foundation, can be found at https://github.com/dotnet/runtime
    /// Equivalent to the Write7BitEncodedInt64 method in C#. 
    pub fn write_7_bit_encoded_int64(&mut self, data: i64) -> io::Result<usize> {
        let mut value = data as u64;
        let mut out_bytes: Vec<u8> = Vec::new();
        while value > 0x7F {
            let low_bits_and_flag: u8 = (value | !0x7F).to_le_bytes()[0];
            value >>= 7;
            out_bytes.push(low_bits_and_flag);
        }
        out_bytes.push(value.to_le_bytes()[0]);
        self.write_bytes(&out_bytes)
    }
    
    /// Equivalent to the Write method in C# called with an argument of type Boolean.
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
    
    /// Equivalent to the Write method in C# called with an argument of type Single
    pub fn write_f32(&mut self, data: f32) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    /// Equivalent to the Write method in C# called with an argument of type Double
    pub fn write_f64(&mut self, data: f64) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    /// Equivalent to the Write method in C# called with an argument of type Half
    #[cfg_attr(docsrs, doc(cfg(feature = "f16")))]
    #[cfg(feature = "f16")]
    pub fn write_f16(&mut self, data: f16) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    /// Equivalent to the Write method in C# called with an argument of type String
    pub fn write_string(&mut self, data: &str) -> io::Result<usize> {
        // first, write the number of bytes the string will take up in utf-8
        if let Err(e) = self.write_7_bit_encoded_int(data.len().try_into().unwrap()) {
            return Err(e)
        }
        // then, write the utf-8 data. rust str is gauranteed to be valid utf-8 so no further
        // processing is needed.
        self.write_bytes(data.as_bytes())
    }
    
    /// Equivalent to the Write method in C# called with an argument of type SByte
    pub fn write_i8(&mut self, data: i8) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    /// Equivalent to the Write method in C# called with an argument of type Int16
    pub fn write_i16(&mut self, data: i16) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    /// Equivalent to the Write method in C# called with an argument of type Int32
    pub fn write_i32(&mut self, data: i32) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    /// Equivalent to the Write method in C# called with an argument of type Int64
    pub fn write_i64(&mut self, data: i64) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    /// Equivalent to the Write method in C# called with an argument of type UInt16
    pub fn write_u16(&mut self, data: u16) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    /// Equivalent to the Write method in C# called with an argument of type UInt32
    pub fn write_u32(&mut self, data: u32) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    /// Equivalent to the Write method in C# called with an argument of type UInt64
    pub fn write_u64(&mut self, data: u64) -> io::Result<usize> {
        self.output.write(&data.to_le_bytes())
    }

    /// Equivalent to the Write method in C# called with an argument of type Char
    pub fn write_char(&mut self, data: char) -> io::Result<usize> {
        let mut buf: [u8; 4] = [0; 4];
        self.write_bytes(data.encode_utf8(buf.as_mut_slice()).as_bytes())
    }

}
