use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct ChunkType {
    bytes: [u8; 4],
}
impl ChunkType {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn is_valid(&self) -> bool {
        self.bytes.iter().all(|&b| b.is_ascii_alphabetic()) && self.is_reserved_bit_valid()
    }
    fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }
    fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }
    fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;
    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        let ct = ChunkType { bytes };
        if !ct.bytes.iter().all(|&b| b.is_ascii_alphanumeric()) {
            return Err(format!("Invalid chunk type: {}", ct));
        }
        Ok(ct)
    }
}
impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.bytes[0] as char,
            self.bytes[1] as char,
            self.bytes[2] as char,
            self.bytes[3] as char
        )
    }
}
impl FromStr for ChunkType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(format!("Chunk type must be 4 characters long, got {}", s));
        }
        let mut bytes = [0; 4];
        for (i, c) in s.chars().enumerate() {
            if !c.is_ascii_alphabetic() {
                return Err(format!("Chunk type must be alphabetic, got {}", s));
            }
            bytes[i] = c as u8;
        }
        Ok(ChunkType { bytes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
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
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
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
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
