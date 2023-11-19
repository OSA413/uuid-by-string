use sha1::{Sha1, Digest};
use md5::{Md5};

fn main() {
    println!("{:?}", generate_uuid("hello world", 3));
    println!("{:?}", generate_uuid("hello world", 5));
}

fn uint8ToHex(ubyte: u8) -> String {
    return format!("{:X?}", ubyte)
}

fn uint8ArrayToHex(buf: &[u8]) -> String {
    let mut result = "".to_owned();
  
    for i in 0..buf.len() {
        result.push_str(uint8ToHex(buf[i]).as_str());
    }
  
    return result;
}

fn hashToUuid(hashBuffer: [u8; 16], version: u8) -> String {
    return format!("{}-{}-{}{}-{}{}-{}", 
        uint8ArrayToHex(&hashBuffer[0..4]),
        uint8ArrayToHex(&hashBuffer[4..6]),
        uint8ToHex((&hashBuffer[6] & 0x0f) | (version * 10)),
        uint8ToHex(hashBuffer[7]),
        uint8ToHex((&hashBuffer[8] & 0x3f) | 0x80),
        uint8ToHex(hashBuffer[9]),
        uint8ArrayToHex(&hashBuffer[10..16])
    )
}

fn md5_hash(buf: &[u8]) -> [u8; 16] {
    let mut hasher = Md5::new();
    hasher.update(buf);
    let result = hasher.finalize();
    return result[0..16].try_into().expect("Wrong length");
}

fn sha1_hash(buf: &[u8]) -> [u8; 16] {
    let mut hasher = Sha1::new();
    hasher.update(buf);
    let result = hasher.finalize();
    return result[0..16].try_into().expect("Wrong length");
}

// TODO: add namespace
fn generate_uuid(target: &str, version: u8) -> Option<String> {
    let result = if version == 3 {
        md5_hash(target.as_bytes())}
    else {
        sha1_hash(target.as_bytes())
    };
    return Some(hashToUuid(result, version));
}
