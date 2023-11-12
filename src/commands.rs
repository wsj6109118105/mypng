use std::{path::PathBuf, str::FromStr};
use std::{fs, path};
use crate::{chunk::{self, Chunk},chunk_type::{self, ChunkType},png::{self, Png}};

/// 将信息编码到图片中
pub fn encode(path: PathBuf, mytype: String, message: String) {
    let mut p = read_from_file(&path);
    p.append_chunk(Chunk::new(ChunkType::from_str(&mytype).unwrap(), message.as_bytes().to_vec()));
    write_to_file(&path, &p.as_bytes());
}

/// 将指定块的内容解码
pub fn decode(path: PathBuf, mytype: String) {
    let mut p = read_from_file(&path);
    let chunk = p.chunk_by_type(&mytype);
    match chunk {
        Some(chunk) => {
            match chunk.data_as_string() {
                Ok(s) => {
                    print!("{}",s);
                },
                _ => print!("未添加消息内容"),
            };
        },
        None => print!("未找到指定块"),
    };
}

/// 移除指定类型块
pub fn remove(path: PathBuf, mytype: String) {
    let mut p = read_from_file(&path);
    let chunk = p.remove_chunk(&mytype);
    match chunk {
        Ok(c) => {
            match c.data_as_string() {
                Ok(s) => print!("移除块的消息为：{}",s),
                _ => print!("移除块中未包含消息"),
            }
        },
        Err(s) => print!("{}",s),
    }
    write_to_file(&path, &p.as_bytes())
}

/// 打印块信息
pub fn print_chunks(path: PathBuf) {
    let p = read_from_file(&path);
    print!("{}",p);
}

fn read_from_file(path: &PathBuf) -> Png {
    let f = fs::read(path).unwrap();
    let png = Png::try_from(f.as_slice()).unwrap();
    png
}

fn write_to_file(path: &PathBuf,date: &[u8]) {
    let _ = fs::write(path, date);
}