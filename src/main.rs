use clap::Parser;
use prost::Message;
use protobuf_testing::citadel_gcmessages_common::{CMsgMatchMetaData, CMsgMatchMetaDataContents};
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

    match decode_match_metadata_from_bytes(&bytes) {
        Ok(contents_bytes) => {
            match decode_match_metadata_contents_from_bytes(contents_bytes.match_details()) {
                Ok(contents) => {
                    println!("match meta data contents: \n{:?}", contents)
                }

                Err(e) => eprintln!("Failed to parse match metadata contents: {}", e),
            };
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

fn decode_match_metadata_from_bytes(bytes: &[u8]) -> Result<CMsgMatchMetaData, prost::DecodeError> {
    CMsgMatchMetaData::decode(bytes)
}

fn decode_match_metadata_contents_from_bytes(
    bytes: &[u8],
) -> Result<CMsgMatchMetaDataContents, prost::DecodeError> {
    CMsgMatchMetaDataContents::decode(bytes)
}
