use crate::chunk_type::ChunkType;
use crc::Crc;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}
impl TryFrom<&[u8]> for Chunk {
    type Error = String;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let length = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let chunk_type = ChunkType::try_from([bytes[4], bytes[5], bytes[6], bytes[7]])?;
        let data = bytes[8..(8 + length as usize)].to_vec();
        let crc = u32::from_be_bytes([
            bytes[8 + length as usize],
            bytes[9 + length as usize],
            bytes[10 + length as usize],
            bytes[11 + length as usize],
        ]);
        if !chunk_type.is_valid() {
            return Err(format!("Invalid chunk type: {}", chunk_type));
        }
        if length != data.len() as u32 {
            return Err(format!(
                "Invalid chunk length: expected {}, got {}",
                length,
                data.len()
            ));
        }
        let expected_crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC)
            .checksum(&[chunk_type.bytes(), data.as_slice()].concat());
        if crc != expected_crc {
            return Err(format!(
                "Invalid chunk CRC: expected {}, got {}",
                expected_crc, crc
            ));
        }
        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk (length: {})", self.length)?;
        write!(f, " (chunk type: {})", self.chunk_type)?;
        write!(f, " (data: {:?})", self.data)?;
        write!(f, " (crc: {})\n", self.crc)?;
        Ok(())
    }
}
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;
        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC)
            .checksum(&[chunk_type.bytes(), data.as_slice()].concat());
        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> ChunkType {
        self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.chunk_type.bytes());
        bytes.extend_from_slice(&self.data);
        bytes.extend_from_slice(&self.crc.to_be_bytes());
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
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

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
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
        let crc: u32 = 2882656333;

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

    #[test]
    pub fn test_chunk_trait_impls() {
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

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
