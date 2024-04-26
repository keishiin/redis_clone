fn parse_data_new(data: &[u8]) -> std::vec::Vec<&str> {
    match data[0] as char {
        '*' => parse_array(data),
        '+' => parse_simple_string(data),
        '$' => parse_bulk_string(data),
        _ => panic!("not supported comomand"),
    }
}

fn parse_simple_string(buffer: &[u8]) -> std::vec::Vec<&str> {
    let filerted_data = filter_data(buffer);
    let cmd = filerted_data[0];
    println!("parse simple string command: {}", cmd);
    filerted_data
}

fn parse_array(buffer: &[u8]) -> std::vec::Vec<&str> {
    let filerted_data = filter_data(buffer);
    println!("parse array string: {:?}", filerted_data);
    filerted_data
}

fn parse_bulk_string(buffer: &[u8]) -> std::vec::Vec<&str> {
    let filerted_data = filter_data(buffer);
    println!("parse bulk string: {:?}", filerted_data);
    filerted_data
}

fn filter_data(buffer: &[u8]) -> std::vec::Vec<&str> {
    let data = std::str::from_utf8(buffer).expect("unable to read from buffer");
    data.trim_end_matches("\r\n")
        .split("\r\n")
        .filter(|&x| x.chars().all(|c| c.is_alphabetic()))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_array_parse() {
        let data = "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        let temp = parse_data_new(data.as_bytes());
        assert_eq!(temp, ["SET", "key", "value"]);
    }

    #[test]
    fn test_simple_string_parse() {
        let data = "+1\r\n$4\r\nping\r\n";
        let temp = parse_data_new(data.as_bytes());
        assert_eq!(temp, ["ping"]);
    }

    #[test]
    fn test_bulk_string_parse() {
        let data = "$6\r\nGET\r\n$5\r\nmykey\r\n";
        let temp = parse_data_new(data.as_bytes());
        assert_eq!(temp, ["GET", "mykey"]);
    }
}
