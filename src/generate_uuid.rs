#[path="./common.rs"]
mod common;

pub fn generate_uuid(target: &str) -> String {
    generate_uuid_v5(target)
}

pub fn generate_uuid_v3(target: &str) -> String {
    let target_char_buffer = target.as_bytes();
    let buffer = target_char_buffer.to_vec();
    let result = common::md5_hash(buffer);
    return common::hash_to_uuid(result, 3);
}

pub fn generate_uuid_v5(target: &str) -> String {
    let target_char_buffer = target.as_bytes();
    let buffer = target_char_buffer.to_vec();
    let result = common::sha1_hash(buffer);
    return common::hash_to_uuid(result, 5);
}
