mod args;
mod chunk;
mod chunk_type;
mod commands;
pub mod png;

use crate::args::PngMeArgs;
use crate::commands::{decode, encode, print_chunks, remove, spoof};

use clap::Parser;
pub use png::Png;

fn main() -> anyhow::Result<()> {
    let args = PngMeArgs::parse();
    println!("{:?}", args);
    match args {
        PngMeArgs::Encode(encode_args) => encode(encode_args),
        PngMeArgs::Decode(decode_args) => decode(decode_args),
        PngMeArgs::Remove(remove_args) => remove(remove_args),
        PngMeArgs::Print(print_args) => print_chunks(print_args),
        PngMeArgs::Spoof(spoof_args) => spoof(spoof_args),
    }
}