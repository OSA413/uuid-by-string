#[path="./common.rs"]
mod common;
#[path="./for_namespace.rs"]
mod for_namespace;

const NIL_UUID: &str = "00000000-0000-0000-0000-000000000000";

pub fn generate(target: &str, namespace: Option<&str>) -> Result<String, &'static str> {
    generate_v5(target, namespace)
}

fn generate_common(target: &str, namespace: Option<&str>, f: &dyn Fn(Vec<u8>) -> [u8; 16], version: u8) -> Result<String, &'static str> {
    let target_char_buffer = target.as_bytes();
    let namespace_char_buffer = for_namespace::parse_uuid(namespace.unwrap_or(NIL_UUID));

    match namespace_char_buffer {
        Err(x) => return Err(x),
        _ => (),
    }

    let namespace_char_buffer = namespace_char_buffer.unwrap();
    let buffer = [&namespace_char_buffer, target_char_buffer].concat();
    let result = f(buffer);
    return Ok(common::hash_to_uuid(result, version));
}

pub fn generate_v3(target: &str, namespace: Option<&str>) -> Result<String, &'static str> {
    generate_common(target, namespace, &common::md5_hash, 3)
}

pub fn generate_v5(target: &str, namespace: Option<&str>) -> Result<String, &'static str> {
    generate_common(target, namespace, &common::sha1_hash, 5)
}
