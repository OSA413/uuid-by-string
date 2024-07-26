#[path="./common.rs"]
mod common;

#[deprecated = "Generating UUID v3 and v5 is non-standard, see RFC-4122"]
pub fn generate(target: &str) -> String {
    #[allow(deprecated)]
    generate_v5(target)
}

#[deprecated = "Generating UUID v3 and v5 is non-standard, see RFC-4122"]
pub fn generate_v3(target: &str) -> String {
    let target_char_buffer = target.as_bytes();
    let buffer = target_char_buffer.to_vec();
    let result = common::md5_hash(buffer);
    return common::hash_to_uuid(result, 3);
}

#[deprecated = "Generating UUID v3 and v5 is non-standard, see RFC-4122"]
pub fn generate_v5(target: &str) -> String {
    let target_char_buffer = target.as_bytes();
    let buffer = target_char_buffer.to_vec();
    let result = common::sha1_hash(buffer);
    return common::hash_to_uuid(result, 5);
}
