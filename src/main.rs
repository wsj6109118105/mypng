mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use clap::Parser;
use args::{Cli,Commonds};
use commands::{encode,decode,remove,print_chunks};
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let arg = Cli::parse();

    match arg.commond {
        Commonds::Encode { path, mytype, message } => encode(path,mytype,message),
        Commonds::Decode { path, mytepe } => decode(path,mytepe),
        Commonds::Remove { path, mytepe } => remove(path,mytepe),
        Commonds::Print { path } => print_chunks(path),
    }

    return Ok(());
}