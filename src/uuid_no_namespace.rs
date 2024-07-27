#[path="./common.rs"]
mod common;

#[deprecated = "Generating UUID v3 and v5 is non-standard, see RFC-4122"]
pub fn generate(target: &str) -> String {
    #[allow(deprecated)]
    generate_v5(target)
}

fn generate_common(target: &str, f: &dyn Fn(Vec<u8>) -> [u8; 16], version: u8) -> String {
    let target_char_buffer = target.as_bytes();
    let buffer = target_char_buffer.to_vec();
    let result = f(buffer);
    return common::hash_to_uuid(result, version);
}

#[deprecated = "Generating UUID v3 and v5 is non-standard, see RFC-4122"]
pub fn generate_v3(target: &str) -> String {
    generate_common(target, &common::md5_hash, 3)
}

#[deprecated = "Generating UUID v3 and v5 is non-standard, see RFC-4122"]
pub fn generate_v5(target: &str) -> String {
    generate_common(target, &common::sha1_hash, 5)
}
