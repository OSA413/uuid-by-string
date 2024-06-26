use regex::Regex;

/** Length of string containing uuid */
const UUID_LENGTH: usize = 36;
/** Regular expression for uuid testing */
const UUID_REGEXP: &str = r"(?i)^[0-9a-f]{8}-[0-9a-f]{4}-[0-5][0-9a-f]{3}-[089ab][0-9a-f]{3}-[0-9a-f]{12}$";

fn is_valid_uuid(uuid: &str) -> bool {
    let regex = Regex::new(UUID_REGEXP).unwrap();
    return uuid.len() == UUID_LENGTH && regex.is_match(uuid);
}

pub fn parse_uuid(uuid: &str) -> Result<[u8; 16], &'static str> {
    if !is_valid_uuid(uuid) {
        return Err("Invalid UUID");
    }

    let mut buf = [0; 16];

    let mut str_index = 0;
    let mut buf_index = 0;

    while str_index < uuid.len() {
        let as_bytes: Vec<char> = uuid.chars().collect();
        if as_bytes[str_index] == '-' {
            str_index += 1;
            continue;
        }

        let oct = 
            u8::from_str_radix(as_bytes[str_index].to_string().as_str(), 16).unwrap() * 0x10
            + u8::from_str_radix(as_bytes[str_index + 1].to_string().as_str(), 16).unwrap();
        buf[buf_index] = oct;

        buf_index += 1;
        str_index += 2;
    }

    return Ok(buf);
}