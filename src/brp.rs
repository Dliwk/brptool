use anyhow::Result;
use binrw::{BinRead, BinReaderExt};
use std::io::{Read, Seek};

use crate::error::BrpError;
use crate::huffman::Huffman;
use crate::session::handle_session_message;

const BRP_FILE_ID: u32 = 83749;
const TARGET_PROTOCOL_VERSION: u16 = 33;

#[derive(BinRead, Debug)]
struct BrpHeader {
    magic: u32,
    protocol_version: u16,
}

fn read_message_length<T: Read>(stream: &mut T) -> Result<u32> {
    // The first byte represents the actual size if the value is < 254
    // if it is 254, the 2 bytes after it represent size
    // if it is 255, the 4 bytes after it represent size
    // (from original ballistica source, logic/session/replay_client_session.cc)

    let mut buf = [0; 1];
    stream.read_exact(&mut buf)?;
    let len = u8::from_le_bytes(buf);
    if len < 254 {
        Ok(len.into())
    } else if len == 254 {
        let mut buf = [0; 2];
        stream.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf).into())
    } else if len == 255 {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    } else {
        unreachable!();
    }
}

fn load_replay_messages<T: Read>(mut stream: T) -> Result<()> {
    let huffman = Huffman::build();
    loop {
        let length = match read_message_length(&mut stream) {
            Ok(r) => r,
            Err(_) => break,
        };
        let mut buf = vec![0; length as usize];
        stream.read_exact(&mut buf)?;
        // println!("msg {} bytes", length);
        let data = huffman.decompress(&buf);
        // println!("data: {:?}", data);
        handle_session_message(&data);
        // break;
    }

    Ok(())
}

pub fn load_replay<T: Read + Seek>(mut stream: T) -> Result<()> {
    let header: BrpHeader = stream.read_ne()?;

    if header.magic != BRP_FILE_ID {
        return Err(BrpError::NotABrpFile.into());
    }

    if header.protocol_version != TARGET_PROTOCOL_VERSION {
        return Err(BrpError::UnsupportedProtocolVersion(header.protocol_version).into());
    }

    load_replay_messages(stream)?;

    Ok(())
}
