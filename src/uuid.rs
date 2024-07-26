#[path="./common.rs"]
mod common;
#[path="./for_namespace.rs"]
mod for_namespace;

const NIL_UUID: &str = "00000000-0000-0000-0000-000000000000";

pub fn generate(target: &str, namespace: Option<&str>) -> Result<String, &'static str> {
    generate_v5(target, namespace)
}

pub fn generate_v3(target: &str, namespace: Option<&str>) -> Result<String, &'static str> {
    let target_char_buffer = target.as_bytes();
    let namespace_char_buffer = for_namespace::parse_uuid(namespace.unwrap_or(NIL_UUID));

    match namespace_char_buffer {
        Err(x) => return Err(x),
        _ => (),
    }

    let namespace_char_buffer = namespace_char_buffer.unwrap();
    let buffer = [&namespace_char_buffer, target_char_buffer].concat();
    let result = common::md5_hash(buffer);
    return Ok(common::hash_to_uuid(result, 3));
}

pub fn generate_v5(target: &str, namespace: Option<&str>) -> Result<String, &'static str> {
    let target_char_buffer = target.as_bytes();
    let namespace_char_buffer = for_namespace::parse_uuid(namespace.unwrap_or(NIL_UUID));

    match namespace_char_buffer {
        Err(x) => return Err(x),
        _ => (),
    }

    let namespace_char_buffer = namespace_char_buffer.unwrap();
    let buffer = [&namespace_char_buffer, target_char_buffer].concat();
    let result = common::sha1_hash(buffer);
    return Ok(common::hash_to_uuid(result, 5));
}
