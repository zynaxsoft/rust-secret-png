use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;


#[derive(Debug, PartialEq)]
pub struct ChunkType {
    data: [u8; 4],
}

impl ChunkType {

    fn is_capital(byte: u8) -> bool {
        (byte & 0b00100000) == 0b00000000
    }

    pub fn new(data: [u8; 4]) -> ChunkType {
        ChunkType {data}
    }

    pub fn bytes(&self) -> &[u8; 4] {
        &self.data
    }

    pub fn is_valid(&self) -> bool {
        for byte in self.data.iter() {
            if !byte.is_ascii_alphabetic() {
                return false
            }
        }
        if !self.is_reserved_bit_valid() {
            return false
        }
        true
    }

    pub fn is_critical(&self) -> bool {
        Self::is_capital(self.data[0])
    }

    pub fn is_public(&self) -> bool {
        Self::is_capital(self.data[1])
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        Self::is_capital(self.data[2])
    }

    pub fn is_safe_to_copy(&self) -> bool {
        !Self::is_capital(self.data[3])
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type = ChunkType::new(value);
        if chunk_type.is_valid() {
            Ok(chunk_type)
        } else {
            println!("Invalid chunk type value {:?}", value);
            Err("Invalid chunk type.")
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = [0; 4];
        for (i, c) in s.chars().enumerate() {
            if !c.is_ascii_alphabetic() {
                println!("The letter {} is not an ascii character", c);
                return Err("Not an ascii character")
            }
            data[i] = c as u8;
        }
        Self::try_from(data)
    }
}

impl fmt::Display for ChunkType {
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
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = &[82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust");
        assert!(chunk.is_err());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }
}
