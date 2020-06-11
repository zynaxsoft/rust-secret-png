use std::convert::TryFrom;
use std::fmt;
use std::convert::From;

use crc::crc32::checksum_ieee;

use crate::chunk_type::ChunkType;


pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;
        let crc = Chunk::calculate_crc(chunk_type.bytes(), data.as_ref());
        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn calculate_crc(chunk_type: &[u8], data: &[u8]) -> u32 {
        let crc_data: Vec<u8> = chunk_type
            .iter()
            .cloned()
            .chain(data.iter().cloned())
            .collect();
        checksum_ieee(crc_data.as_ref())
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, ()> {
        Ok(self.to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .cloned()
            .chain(
                self.chunk_type
                .bytes()
                .iter()
                .cloned()
                )
            .chain(
                self.data
                .iter()
                .cloned()
                )
            .chain(
                self.crc
                .to_be_bytes()
                .iter()
                .cloned()
                )
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            return Err("Raw chunk data length less than 12")
        }
        let mut raw_iter = value.iter();
        let mut length = [0; 4];
        for i in 0..4 {
            length[i] = *raw_iter.next().unwrap();
        }
        let length = u32::from_be_bytes(length);
        let mut chunk_type = [0; 4];
        for i in 0..4 {
            chunk_type[i] = *raw_iter.next().unwrap();
        }
        let chunk_type = ChunkType::try_from(chunk_type)?;
        let mut data = Vec::new();
        for _ in 0..length {
            data.push(*raw_iter.next().unwrap());
        }
        let mut provided_crc = [0; 4];
        for i in 0..4 {
            provided_crc[i] = *raw_iter.next().unwrap();
        }
        let provided_crc = u32::from_be_bytes(provided_crc);
        let calculated_crc = Chunk::calculate_crc(chunk_type.bytes(), data.as_ref());
        if calculated_crc != provided_crc {
            return Err("Invalid crc")
        }
        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc: provided_crc,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in self.data.iter() {
            f.write_fmt(format_args!("{}", char::from(c.clone()))).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data: Vec<u8> = "This is where your secret message will be!"
            .bytes()
            .collect();
        Chunk::new(chunk_type, data)
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 111111111;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }
}
