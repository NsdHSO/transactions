use std::io::{Cursor, Read};

use anyhow::Result;
use tokio;

fn find_block(_message: &[u8], _ublock_id: u64) {
    todo!()
}

struct BlockReader<'a> {
    message: Vec<u8>,
    block_reader: Cursor<&'a [u8]>,
}

impl Read for BlockReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.block_reader.read(buf)
    }
}

fn new_block_reader(message: Vec<u8>, block_id: u64) -> BlockReader {
    let (pos, count) = find_block(&message, block_id);

    let slice = &message[pos..(pos + count)];

    BlockReader {
        message,
        block_reader: Cursor::new(slice),
    }
}
fn main() -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        println!("Reading messages...");
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}
