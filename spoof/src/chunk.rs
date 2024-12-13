use std::error::Error;
use std::fmt::{self, Formatter};
use std::io::{BufReader, Read};
use crate::chunk_type::ChunkType;
use crc32fast::Hasher;

// Custom Error Types
#[derive(Debug)]
pub enum ChunkError {
    ChunkNotFound(String),
    ChunkLength(String),
}

// Display trait for our custom error.
impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkError::ChunkNotFound(chunk_type) => write!(f, "Chunk with type '{}' does not exist", chunk_type),
            ChunkError::ChunkLength(chunk_message) => write!(f, "{}", chunk_message),
        }
    }
}

// Chunk Layout that consists of four parts.
#[derive(Debug)]
pub struct Chunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub chunk_data: Vec<u8>,
    pub crc: u32,
}

impl Chunk {
    // Creating a new chunk from a chunk_type and chunk_data
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let data_len = data.len() as u32;
        let chunk_type_bytes = chunk_type.chunk_type;

        let mut hasher = Hasher::new();
        hasher.update(chunk_type_bytes.as_ref());
        hasher.update(&data);

        let checksum: u32 = hasher.finalize();

        Chunk {
            length: data_len,
            chunk_type,
            chunk_data: data,
            crc: checksum,
        }
    }

    // Length data of chunk
    pub fn length(&self) -> u32 {
        self.length
    }

    // Chunk type
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    // Chunk data
    pub fn data(&self) -> &[u8] {
        self.chunk_data.as_ref()
    }

    // Check sum
    pub fn crc(&self) -> u32 {
        self.crc
    }

    // Chunk data as string
    pub fn data_as_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.chunk_data.iter().map(|&x| x as char).collect())
    }

    // Chunk Layout as bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        let data_len = self.length.to_be_bytes();
        let crc_be = self.crc.to_be_bytes();
        data_len.iter().chain(self.chunk_type.chunk_type.iter()).chain(self.chunk_data.iter()).chain(crc_be.iter()).copied().collect::<Vec<u8>>()
    }
}

// Type conversion of a Chunk from a primitive (byte) array
impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;
    fn try_from(text: &[u8]) -> Result<Self, Self::Error> {
        if text.len() < 8 {
            return Err(ChunkError::ChunkLength(format!("Bytes lenght is less than eight")));
        }
        let mut reader = BufReader::new(text);
        let mut buffer: [u8; 4] = [0, 0, 0, 0];

        reader.read_exact(&mut buffer).unwrap();
        let data_length = u32::from_be_bytes(buffer);

        reader.read_exact(&mut buffer).unwrap();
        let chunk_type = ChunkType::try_from(buffer).unwrap();

        let mut data: Vec<u8> = vec![0; data_length as usize];
        reader.read_exact(&mut data).unwrap();

        reader.read_exact(&mut buffer).unwrap();
        let crc = u32::from_be_bytes(buffer);

        Ok(Chunk {
            length:  data_length,
            chunk_type: chunk_type,
            chunk_data: data,
            crc
        })
    }
}

// Display Trait for Chunk
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk {{ ")?;
        writeln!(f)?;
        write!(f, "\tLength: {}\n", self.length)?;
        write!(f, "\tChunk Type: {} \n", self.chunk_type)?;
        write!(f, "\tChunk Data: {} \n", self.chunk_data.len())?;
        writeln!(f, "\tCRC: {}", self.crc)?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

#[allow(unused_variables)]
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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
        
        let _chunk_string = format!("{:?}", chunk);
    }
}

