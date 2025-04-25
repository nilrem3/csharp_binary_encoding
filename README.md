# csharp_binary_encoding
A crate for handling binary data in the format used by the C# [`System.IO.BinaryReader`] and [`System.IO.BinaryWriter`] Classes.
## Features
- `f16` Enables function for decoding f16 values. Must be compiled with nightly, since f16 is currently an unstable feature in rust.
## Example
### Reading values
```
# use csharp_binary_encoding::{BinaryReader, DataDecodeError};
// &[u8] implements read
let bytes: [u8; 11] = [ 0x8F, 0x72, 0x04, 0x6D, 0x65, 0x6F, 0x77, 0xD7, 0xA3, 0xE8, 0x40 ];

// Construct a reader
let mut reader = BinaryReader::new(bytes.as_slice());

// Read values
assert_eq!(14607, reader.read_7_bit_encoded_int()?);
assert_eq!("meow".to_string(), reader.read_string()?);
assert_eq!(7.27_f32, reader.read_f32()?);
# Ok::<(), DataDecodeError>(())
```
### Writing values
```
# use csharp_binary_encoding::BinaryWriter;
// Vec<u8> implements write
let mut data: Vec<u8> = Vec::new();

// Construct a writer
let mut writer = BinaryWriter::new(&mut data);

//write values
writer.write_7_bit_encoded_int(14607)?;
writer.write_string("meow")?;
writer.write_f32(7.27_f32)?;

assert_eq!(
    data,
    vec![ 0x8F, 0x72, 0x04, 0x6D, 0x65, 0x6F, 0x77, 0xD7, 0xA3, 0xE8, 0x40 ]
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Limitations
- Currently, only the utf-8 encoding is supported.
- Developed for and tested with .NET version 9.0. Compatibility with other versions is likely
  present but should not be counted on.

## Contributions
- Some methods in this library were translated directly from the [dotnet runtime library]'s implementation. For details see the CONTRIBUTIONS file.

[`System.IO.BinaryWriter`]: <https://learn.microsoft.com/en-us/dotnet/api/system.io.binarywriter>
[`System.IO.BinaryReader`]: <https://learn.microsoft.com/en-us/dotnet/api/system.io.binaryreader>
[dotnet runtime library]: <https://github.com/dotnet/runtime>
