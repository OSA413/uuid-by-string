#[path="./common.rs"]
mod common;

pub fn generate_uuid(target: &str, version: u8) -> String {
    let target_char_buffer = target.as_bytes();

    let buffer = target_char_buffer.to_vec();

    let result = if version == 3 {
        common::md5_hash(buffer)}
    else {
        common::sha1_hash(buffer)
    };
    return common::hash_to_uuid(result, version);
}
