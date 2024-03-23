
use std::fs::File;
use std::io::{self, BufRead, Read};
use rlp::{decode, Rlp};


fn main() -> io::Result<()> {
    // Specify the path to the file in the parent directory
    let file_path = "../export_receipt_0_4061223";

    // Synchronously read the file contents
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Attempt to decode the RLP data
    let decoded = Rlp::new(buffer.as_slice());
    if decoded.is_list() {
        // Handle list decoding, if needed
        println!("Decoded a list of items:");
        for item in decoded.iter() {
            println!("{:?}", item.as_raw());
        }
    } else {
        // Assume it's a single item and just print it
        let bytes = decoded.as_raw();
        println!("Decoded data: {:?}", bytes);
    }

    Ok(())
}
