# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased
- Add num_bytes_read method to BinaryReader
- Add num_bytes_written method to BinaryReader
- Change licensing information to be more clear about functions translated from dotnet

## [0.3.1]
- Fixed a bug where BinaryReader would not correctly read the null byte
- Added a test to check for this

## [0.3.0]
- Renamed DataDecodeError to InvalidDataError to more accurately indicate the category of error represented by this type
- Create InvalidDataError to represent one of the types of DataDecodeError
- Added an example for writing data to README.md

## [0.2.0]
- Added BinaryWriter struct
- Added a test that decodes data written by rust in C#
- Changed the name of the C# testing folder

## [0.1.1]
- Fix typo in Cargo.toml enabling docs.rs to correctly process documentation

## [0.1.0]

- Added BinaryReader struct
- Added DataDecodeError enum
- Added f16 feature
- Added basic C# program to generate test binary data
- Added decode_all_types test
- Added overflow_7_bit_encoded_int test
- Added basic documentation
- Added this changelog

[0.3.1]: <https://github.com/nilrem3/csharp_binary_encoding/compare/v0.3.0...v0.3.1>
[0.3.0]: <https://github.com/nilrem3/csharp_binary_encoding/compare/v0.2.0...v0.3.0>
[0.2.0]: <https://github.com/nilrem3/csharp_binary_encoding/compare/v0.1.1...v0.2.0>
[0.1.1]: <https://github.com/nilrem3/csharp_binary_encoding/compare/v0.1.0...v0.1.1>
[0.1.0]: <https://github.com/nilrem3/csharp_binary_encoding/releases/tag/v0.1.0>
