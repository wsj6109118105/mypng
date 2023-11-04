use std::str::FromStr;
use std::fmt;
use std::str;
#[derive(PartialEq,Eq,Debug)]
pub struct ChunkType {
    pub chunk_type: Vec<u8>
}

/// 实现从u8数组转换为一个块类型，未进行错误处理
impl TryFrom<[u8;4]> for ChunkType {

    type Error = &'static str;

    fn try_from(value: [u8;4]) -> Result<Self, Self::Error> {
        let mut chunktype = ChunkType{
            chunk_type: Vec::new()
        };
        for i in value {
            chunktype.chunk_type.push(i);
        }
        if !((chunktype.chunk_type[0].is_ascii_lowercase() || chunktype.chunk_type[0].is_ascii_uppercase()) &&
        (chunktype.chunk_type[1].is_ascii_lowercase() || chunktype.chunk_type[1].is_ascii_uppercase()) &&
        (chunktype.chunk_type[3].is_ascii_lowercase() || chunktype.chunk_type[3].is_ascii_uppercase()) &&
        (chunktype.chunk_type[2].is_ascii_uppercase() || chunktype.chunk_type[2].is_ascii_lowercase())) {
            Err("字节数组非法")
        }else {
            Ok(chunktype)
        }
    }
}

/// 实现从字符串转换为块类型
impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunktype = ChunkType {
            chunk_type: Vec::new()
        };
        for i in s.bytes() {
            chunktype.chunk_type.push(i);
        }
        if !((chunktype.chunk_type[0].is_ascii_lowercase() || chunktype.chunk_type[0].is_ascii_uppercase()) &&
        (chunktype.chunk_type[1].is_ascii_lowercase() || chunktype.chunk_type[1].is_ascii_uppercase()) &&
        (chunktype.chunk_type[3].is_ascii_lowercase() || chunktype.chunk_type[3].is_ascii_uppercase()) &&
        (chunktype.chunk_type[2].is_ascii_uppercase() || chunktype.chunk_type[2].is_ascii_lowercase())) {
            Err("块类型非法")
        }else {
            Ok(chunktype)
        }
    }
}

/// 实现display特性，以字符串形式打印
impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = str::from_utf8(&self.chunk_type).unwrap();
        write!(f, "{}",s)
    }
}

/// 结构体实现方法
impl ChunkType {
    // 转换为字节数组
    fn bytes(&self) -> [u8; 4] {
        let a:[u8;4] = [self.chunk_type[0],self.chunk_type[1],self.chunk_type[2],self.chunk_type[3]];
        a
    }

    // 校验块类型是否合法
    fn is_valid(&self) -> bool {
        (self.chunk_type[0].is_ascii_lowercase() || self.chunk_type[0].is_ascii_uppercase()) &&
        (self.chunk_type[1].is_ascii_lowercase() || self.chunk_type[1].is_ascii_uppercase()) &&
        (self.chunk_type[3].is_ascii_lowercase() || self.chunk_type[3].is_ascii_uppercase()) &&
        (self.chunk_type[2].is_ascii_uppercase())
    }

    // 是否是关键块，第一字节，第5bit   0:关键块   1:辅助块
    fn is_critical(&self) -> bool {
        self.chunk_type[0].is_ascii_uppercase()
    }

    // 是否为公开块，第二字节，第5bit   0:公开     1:私有
    fn is_public(&self) -> bool {
        self.chunk_type[1].is_ascii_uppercase()
    }

    // 预留位是否正确，第三字节，第5bit 0:预留   此位必须为0
    fn is_reserved_bit_valid(&self) -> bool {
        self.chunk_type[2].is_ascii_uppercase()
    }

    // 是否能安全复制，第四字节，第5bit 0:不安全     1:安全
    fn is_safe_to_copy(&self) -> bool {
        self.chunk_type[3].is_ascii_lowercase()
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