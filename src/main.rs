use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;
mod args;
mod chunk;
mod chunk_type;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
fn main() -> Result<()> {
    let args = args::Args::parse();
    let mut file = File::open(args.file)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;
    let mut png = png::Png::try_from(contents.as_slice())?;
    if args.cmd == "encode" {
        png.append_chunk(chunk::Chunk::new(
            chunk_type::ChunkType::from_str(&args.chunk_type)?,
            args.message.as_bytes().to_vec(),
        ));
    } else if args.cmd == "decode" {
        png.chunk_by_type(&args.chunk_type)
            .map(|chunk| println!("{}", chunk.data_as_string().unwrap()));
    } else if args.cmd == "remove" {
        png.remove_chunk(&args.chunk_type).unwrap();
    } else if args.cmd == "print" {
        println!("{}", png);
    } else {
        Err("Invalid command")?;
    }
    let mut ofile = File::create(&args.output)?;
    ofile.write_all(png.as_bytes().as_slice())?;
    Ok(())
}

