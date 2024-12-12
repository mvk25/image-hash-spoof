use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "pngme")]
#[command(about = "A PNG message encoder/decoder", long_about = None)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
    Spoof(SpoofArgs),
}


// #[derive(Subcommand, Debug)]
// pub enum Commands {
//     Encode(EncodeArgs),
//     Decode(DecodeArgs),
//     Remove(RemoveArgs),
//     Print(PrintArgs),
// }

#[derive(Parser, Debug)]
pub struct SpoofArgs {
    pub hex: String,
    pub og_file: PathBuf,
    pub altered: PathBuf
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    pub file: PathBuf,
    pub chunk: String,
    pub message: String,
    pub out: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    pub file: PathBuf,
    pub chunk: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    pub file: PathBuf,
    pub chunk: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    pub file: PathBuf,
}
