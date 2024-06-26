use sha1::{Sha1, Digest};
use md5::Md5;

fn uint8_to_hex(ubyte: u8) -> String {
    return if ubyte < 16
        { format!("0{:x?}", ubyte) }
        else {format!("{:x?}", ubyte)}
}

fn uint8_array_to_hex(buf: &[u8]) -> String {
    let mut result = "".to_owned();
  
    for i in 0..buf.len() {
        result.push_str(uint8_to_hex(buf[i]).as_str());
    }
  
    return result;
}

pub fn hash_to_uuid(hash_buffer: [u8; 16], version: u8) -> String {
    return format!("{}-{}-{}{}-{}{}-{}", 
        // The low field of the timestamp
        uint8_array_to_hex(&hash_buffer[0..4]),
        // The middle field of the timestamp
        uint8_array_to_hex(&hash_buffer[4..6]),
        // The high field of the timestamp multiplexed with the version number
        uint8_to_hex((&hash_buffer[6] & 0x0f) | (version * 0x10)),
        uint8_to_hex(hash_buffer[7]),
        // The high field of the clock sequence multiplexed with the variant
        uint8_to_hex((&hash_buffer[8] & 0x3f) | 0x80),
        // The low field of the clock sequence
        uint8_to_hex(hash_buffer[9]),
        //  The spatially unique node identifier
        uint8_array_to_hex(&hash_buffer[10..16])
    )
}

pub fn md5_hash(buf: Vec<u8>) -> [u8; 16] {
    let mut hasher = Md5::new();
    hasher.update(buf);
    let result = hasher.finalize();
    return result[0..16].try_into().expect("Wrong length");
}

pub fn sha1_hash(buf: Vec<u8>) -> [u8; 16] {
    let mut hasher = Sha1::new();
    hasher.update(buf);
    let result = hasher.finalize();
    return result[0..16].try_into().expect("Wrong length");
}