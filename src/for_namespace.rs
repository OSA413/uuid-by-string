use regex::Regex;

/** List of hex digit for fast accessing by index */
const HEX_DIGITS: &[char; 16] = &['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];
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
        let as_bytes = uuid.as_bytes();
        if as_bytes[str_index] == b'-' {
            str_index += 1;
            continue;
        }

        let oct = as_bytes[str_index] + as_bytes[str_index + 1];
        buf[buf_index] = oct;

        buf_index += 1;
        str_index += 2;
    }

    return Ok(buf);
}