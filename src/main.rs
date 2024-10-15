use clap::Parser;
use prost::Message;
use protobuf_testing::citadel_gcmessages_common::CMsgMatchMetaDataContents;
use snap::read::FrameDecoder;
use std::fs::File;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let bytes = if let Some(file) = args.file {
        read_protobuf_file(&file)
    } else {
        read_stdin()
    }?;

    match decompress_snappy_data(&bytes) {
        Ok(decompressed_data) => {
            println!("data: {:?}", decompressed_data);
        }
        Err(e) => eprintln!("Decompression failed: {}", e),
    }

    match decode_match_metadata_from_bytes(&bytes) {
        Ok(contents) => {
            println!("Contents: {:?}", contents);
        }
        Err(e) => eprintln!("Failed to parse file: {}", e),
    }

    Ok(())
}

fn read_protobuf_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn read_stdin() -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn decode_match_metadata_from_bytes(
    bytes: &[u8],
) -> Result<CMsgMatchMetaDataContents, prost::DecodeError> {
    CMsgMatchMetaDataContents::decode(bytes)
}

fn decompress_snappy_data(compressed_data: &[u8]) -> io::Result<Vec<u8>> {
    let mut decompressor = FrameDecoder::new(compressed_data);
    let mut buffer = Vec::new();
    decompressor.read_to_end(&mut buffer)?;
    Ok(buffer)
}
