use std::str::from_utf8;
use std::vec::Vec;

pub fn parse_data_new(data: &[u8]) -> Option<std::vec::Vec<&str>> {
    match data[0] as char {
        '*' => filter_data(data),
        '+' => filter_data(data),
        '$' => filter_data(data),
        _ => None,
    }
}

fn filter_data(buffer: &[u8]) -> Option<Vec<&str>> {
    let data_in_utf8 = from_utf8(buffer).expect("unable to read from buffer");
    let filtered: Vec<&str> = data_in_utf8
        .trim_end_matches("\r\n")
        .split("\r\n")
        .filter(|&x| x.chars().all(|c| c.is_alphabetic()))
        .collect();

    if filtered.is_empty() {
        None
    } else {
        Some(filtered)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_set_parse() {
        let test_data = "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        let temp = parse_data_new(test_data.as_bytes());
        let v = vec!["SET", "key", "value"];
        assert_eq!(temp, Some(v));
    }

    #[test]
    fn test_set_parse_multiple_key_values() {
        let test_data = "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n$3\r\nkey\r\n$5\r\nvalue\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        let temp = parse_data_new(test_data.as_bytes());
        let v = vec!["SET", "key", "value", "key", "value", "key", "value"];
        assert_eq!(temp, Some(v));
    }

    #[test]
    fn test_ping_string_parse() {
        let test_data = "+1\r\n$4\r\nping\r\n";
        let temp = parse_data_new(test_data.as_bytes());
        let v = vec!["ping"];
        assert_eq!(temp, Some(v));
    }

    #[test]
    fn test_get_value() {
        let test_data = "$6\r\nGET\r\n$5\r\nmykey\r\n";
        let temp = parse_data_new(test_data.as_bytes());
        let v = vec!["GET", "mykey"];
        assert_eq!(temp, Some(v));
    }
}
