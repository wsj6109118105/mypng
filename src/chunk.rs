use crate::chunk_type::ChunkType;
use std::fmt;
use std::str;
use crc;
use crc::Crc;
pub struct Chunk {
    length: u32,
    ctype: ChunkType,
    date: Vec<u8>,
    chcrc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let l = value.len();
        // 字节序列不能少于12字节
        if l < 12 {
            Err("图片字节序列出错")
        }else {
            let mut chunk = Chunk {
                length: 0,
                ctype: ChunkType{chunk_type: Vec::new()},
                date: Vec::new(),
                chcrc: 0,
            };
            let len = [value[0],value[1],value[2],value[3]];
            let ty = [value[4],value[5],value[6],value[7]]; 
            chunk.length = u32::from_be_bytes(len);
            chunk.ctype = ChunkType::try_from(ty).unwrap();
            for i in 8..l-4 {
                chunk.date.push(value[i]);
            }
            let cr = [value[l-4],value[l-3],value[l-2],value[l-1]];
            chunk.chcrc = u32::from_be_bytes(cr);
            // 计算实际crc校验码
            let c:Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
            let cdate:Vec<u8> = chunk.ctype.chunk_type.iter().chain(chunk.date.iter()).copied().collect();
            let real_crc = c.checksum(&cdate);
            // 查看图片校验码是否正确
            if real_crc == chunk.chcrc {
                Ok(chunk)
            }else {
                Err("图片crc校验码出错")
            }
        }
    }
}

/// 仅打印了数据区的内容，未打印长度，块类型
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = str::from_utf8(&self.date).unwrap();
        write!(f, "{}",s)
    }
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let c:Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let crcdate:Vec<u8> = chunk_type.chunk_type.iter().chain(data.iter()).cloned().collect();
        Chunk { length: data.len() as u32, ctype: chunk_type, date: data, chcrc: c.checksum(&crcdate) }
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.ctype
    }

    fn data(&self) -> &[u8] {
        &self.date
    }

    fn crc(&self) -> u32 {
        self.chcrc
    }

    fn data_as_string(&self) -> Result<String,()> {
        let data = self.data().clone();
        let s = String::from_utf8(data.to_vec()).unwrap();
        Ok(s)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let chunk_bytes:Vec<u8> = self.length.to_be_bytes().iter().
        chain(self.ctype.chunk_type.iter()).
        chain(self.date.iter()).
        chain(self.chcrc.to_be_bytes().iter()).
        copied().
        collect();
        chunk_bytes
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
        
        let _chunk_string = format!("{}", chunk);
    }
}