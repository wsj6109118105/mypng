use std::path::PathBuf;
use clap::{Parser, Subcommand, ValueEnum};


#[derive(Debug,Parser)]
#[command(name="mypng")]
pub struct Cli{
    #[command(subcommand)]
    pub commond: Commonds,
}

#[derive(Debug,Subcommand)]
pub enum Commonds {
    /// 编码信息到图片
    #[command(arg_required_else_help = true)]
    Encode {
        /// 图片路径
        path: PathBuf,
        /// 请指定块类型，eg:ruSt，第一个字符必须为小写，第三个字符必须为大写
        mytype: String,
        /// 要编码的信息
        message: String,
    },

    /// 解码保存在块中的信息
    #[command(arg_required_else_help = true)]
    Decode {
        /// 图片路径
        path: PathBuf,
        /// 要解码信息的块类型
        mytepe: String,
    },

    /// 移除保存编码信息的块
    #[command(arg_required_else_help = true)]
    Remove {
        /// 图片路径
        path: PathBuf,
        /// 要移除的块信息
        mytepe: String,
    },

    /// 打印图片字节信息
    #[command(arg_required_else_help = true)]
    Print {
        /// 图片路径
        path: PathBuf,
    }
}