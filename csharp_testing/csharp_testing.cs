using System;
using System.IO;
using System.Text;

using Newtonsoft.Json;

// Exit Codes:
// 0: Verify Mode: bin was valid. Generate mode: Generated bin.
// 1: Verify mode: bin was not valid
// 2: Invalid command line arguments

if (args.Length < 1) {
    Console.WriteLine("No Mode Provided! Quitting.");
    Environment.Exit(2);
}

if (args[0] == "generate") {
    GenerateTestBin();
    Environment.Exit(0);
} else if (args[0] == "verify") {
    bool verifyf16 = args.Length >= 2 && args[1] == "f16";
    int result = VerifyTestBin(verifyf16);
    if (result == 1) {
        Console.WriteLine("Invalid Binary!");
    }
    Environment.Exit(result);
} else {
    Environment.Exit(2);
}

bool ArrayAssertEq<T>(T[] a, T[] b) where T: IEquatable<T> {
    if (a == null || b == null) {
        return a == b; // true if both are null and false otherwise
    }
    if (a.Length != b.Length) {
        return false; 
    }
    return a.Zip(b, Tuple.Create).Select(pair => AssertEq(pair.Item1, pair.Item2)).All(item => item);
}

bool AssertEq<T>(T a, T b) where T: IEquatable<T> {
    if (a == null) {
        return false;
    }
    if (a.Equals(b)) return true;
    Console.WriteLine($"Assertion Failed on type {typeof(T).FullName}! Expected {JsonConvert.SerializeObject(b)} but got {JsonConvert.SerializeObject(a)}!");
    return false;
}

int VerifyTestBin(bool verifyf16) {
    using (var stream = File.Open("input.bin", FileMode.Open)) {
        using (var reader = new BinaryReader(stream, Encoding.UTF8, false)) {
            if(!AssertEq(reader.ReadBoolean(), true)) return 1;
            if(!AssertEq(reader.ReadBoolean(), false)) return 1;
            if(!AssertEq(reader.ReadByte(), 0x45)) return 1;
            if(!ArrayAssertEq (reader.ReadBytes(5), new byte[] { 0x01, 0x02, 0x03, 0x04, 0x05 })) return 1;
            if(!AssertEq(reader.ReadChar(), '\u2603')) return 1;
            if(!AssertEq(reader.ReadDouble(), 727.247d)) return 1;
            Half h = reader.ReadHalf();
            if (verifyf16) {
                if(!AssertEq(h, (Half)247)) return 1;
            } 
            if(!AssertEq(reader.ReadInt16(), (short)-5)) return 1;
            if(!AssertEq(reader.ReadInt32(), (int)-100)) return 1;
            if(!AssertEq(reader.ReadInt64(), (long) -2147483649)) return 1;
            if(!AssertEq (reader.ReadSByte(), (sbyte)-112)) return 1;
            if(!AssertEq(reader.ReadSingle(), 5.2f)) return 1;
            if(!AssertEq(reader.ReadString(), "meowmeowmeowmeowmeow")) return 1;
            if(!AssertEq(reader.ReadUInt16(), (ushort)624)) return 1;
            if(!AssertEq(reader.ReadUInt32(), (uint)3000000000)) return 1;
            if(!AssertEq(reader.ReadUInt64(), (ulong)42307830165)) return 1;
            if(!AssertEq(reader.Read7BitEncodedInt(), -723)) return 1;
            if(!AssertEq(reader.Read7BitEncodedInt(), 404)) return 1;
            if(!AssertEq(reader.Read7BitEncodedInt64(), 9000000000000000000)) return 1;
            if(!AssertEq(reader.Read7BitEncodedInt64(), -500000000000000000)) return 1;
            return 0;
        }
    }
}

void GenerateTestBin() {
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
        }
    }
}
