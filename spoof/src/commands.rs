use std::convert::TryFrom;
use std::fs;
use std::io::Write;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs, SpoofArgs};
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use sha2::{Sha512, Digest};
 
// Add a ancillary chunk type and create a nonce as the chunk data.
// You can increment the nonce to hit the target of the hash you want to
// achieve. This is similar to Bitcoin's target/difficulty for a block.
pub fn spoof(args: SpoofArgs) -> anyhow::Result<()> {
    let mut png = Png::from_file(args.og_file)?;
    let mut nonce: u32 = 1;
    let mut result = String::new();
    
    loop {
        // Create a new chunk with the ancillary bit set.
        let chunk_type = ChunkType::from_str("rUSt").unwrap();
        let data = nonce.to_be_bytes().to_vec();
        
        // Append this chunk in our png file.
        png.append_chunk(Chunk::new(chunk_type, data));
        let mut hasher = Sha512::new();
        hasher.update(png.as_bytes());
        result = hex::encode(hasher.finalize());
        
        // Check for our condition.
        if result.starts_with(&args.hex[2..]) {
            let mut altered_file = fs::File::create(&args.altered).unwrap();
            altered_file.write(&png.as_bytes()).unwrap();
            break;
        } else {
            png.chunks.pop();
            nonce += 1;
        }
    }

    println!("Nonce: {}\nResulting Hash: {}", nonce, result);
    return Ok(());
    

}

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> anyhow::Result<()> {
    let mut png = Png::from_file(&args.file)?;

    let chunk_type = ChunkType::from_str(&args.chunk).unwrap();
    let data = args.message.into_bytes();

    png.append_chunk(Chunk::new(chunk_type, data));

    let file_path = match args.out {
        Some(path) => path,
        None => args.file,
    };

    fs::write(&file_path, &png.as_bytes())?;

    println!("Wrote message to: {:?}", &file_path);

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> anyhow::Result<()> {
    let png = Png::from_file(&args.file)?;

    match png.chunk_by_type(&args.chunk) {
        Some(message_chunk) => {
            let message = std::str::from_utf8(message_chunk.data())?;
            println!("{}", message);
        }
        None => println!("Error: No chunk of type {}", &args.chunk),
    }

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> anyhow::Result<()> {
    let mut png = Png::from_file(&args.file)?;
    png.remove_chunk(&args.chunk)?;
    fs::write(&args.file, &png.as_bytes())?;
    println!("Removed message from: {:?}", &args.file);

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.file)?;
    let png = Png::try_from(bytes.as_ref())?;
    println!("{}", png);

    Ok(())
}
