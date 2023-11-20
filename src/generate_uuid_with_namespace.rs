#[path="./common.rs"]
mod common;
#[path="./for_namespace.rs"]
mod for_namespace;

pub fn generate_uuid_with_namespace(target: &str, namespace: &str, version: u8) -> Result<String, &'static str> {
    let target_char_buffer = target.as_bytes();
    let namespace_char_buffer = for_namespace::parse_uuid(namespace);

    match namespace_char_buffer {
        Ok(_) => (),
        Err(x) => return Err(x),
    }

    let namespace_char_buffer = namespace_char_buffer.unwrap();

    let buffer = [&namespace_char_buffer, target_char_buffer].concat();

    let result = if version == 3 {
        common::md5_hash(buffer)}
    else {
        common::sha1_hash(buffer)
    };
    return Ok(common::hash_to_uuid(result, version));
}
