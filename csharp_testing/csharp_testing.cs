using System;
using System.IO;
using System.Text;


using (var stream = File.Open("output.bin", FileMode.Create)) {
using (var writer = new BinaryWriter(stream, Encoding.UTF8, false)) {


// write boolean
writer.Write(true);
writer.Write(false);
// write individual byte
writer.Write((byte)0x45);
// write byte array
var bytes = new byte[] { 0x01, 0x02, 0x03, 0x04, 0x05 };
writer.Write(bytes);
// write individual char
writer.Write('\u2603');
// write an individual f64
writer.Write(727.247d);
// write an individual f16
writer.Write((Half)247);
// write an individual i16
writer.Write((short)(-5));
// write an individual i32
writer.Write((int)-100);
// write an individual i64
writer.Write((long)-2147483649);
// write an individual i8
writer.Write((sbyte)-112);
// write an individual f32
writer.Write(5.2f);
// write an individual string
writer.Write("meowmeowmeowmeowmeow");
// write an individual u16
writer.Write((ushort)624);
// write an individual u32
writer.Write((uint)3000000000);
// write an individual u64
writer.Write((ulong)42307830165);
// write an individual 7-bit encoded i32
writer.Write7BitEncodedInt(-723);
writer.Write7BitEncodedInt(404);
// write an individual 7-bit encoded i64
writer.Write7BitEncodedInt64(9000000000000000000);
writer.Write7BitEncodedInt64(-500000000000000000);
}}
