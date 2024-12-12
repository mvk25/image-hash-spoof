use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct ChunkType {
    pub chunk_type: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
    }

    pub fn is_valid(&self) -> bool {
        if self.is_reserved_bit_valid() && self.chunk_type.into_iter().all(|x| x.is_ascii_alphabetic()) {
            true
        } else {
            false
        }
    }

    pub fn is_critical(&self) -> bool {
        if self.chunk_type[0] & 32u8 == 32 {
            false
        } else {
            true
        }
    }

    pub fn is_public(&self) -> bool {
        if self.chunk_type[1] & 32u8 == 32 {
            false
        } else {
            true
        }
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        if self.chunk_type[2] & 32u8 == 32 {
            false
        } else {
            true
        }
    }

    pub fn is_safe_to_copy(&self) -> bool {
        if self.chunk_type[3] & 32u8 == 32 {
            true
        } else {
            false
        }
    }

}


impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.len() == 4 {
            Ok(ChunkType {
                chunk_type: value
            })
        } else {
            Err(format!("ChunkType only accepts arrays of len 4"))
        }
    }
}

impl FromStr for ChunkType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|x| x.is_alphabetic()) {

            let bytes = s.as_bytes();
            let mut array = [0u8; 4];
            
            let len = array.len().min(bytes.len());
            array[..len].copy_from_slice(&bytes[..len]);
            return Ok(
                ChunkType {
                    chunk_type: array
                }
            );
        }

        Err(format!("Does not contain all alphabets"))
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.chunk_type))
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.chunk_type.into_iter().zip(other.chunk_type.into_iter()).all(|x| x.0 == x.1)
    }
}

#[allow(unused_variables)]
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

