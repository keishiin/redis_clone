fn parse_data_new(data: &[u8]) {
    match data[0] as char {
        '*' => parse_array(data),
        '+' => parse_simple_string(data),
        '$' => parse_bulk_string(data),
        _ => panic!("not supported comomand"),
    }
}

fn parse_simple_string(buffer: &[u8]) {
    let data = std::str::from_utf8(buffer).expect("unable to read from buffer");
    let filtered_data: Vec<&str> = data.trim_end_matches("\r\n").split("\r\n").collect();
    let cmd = filtered_data[2];

    println!("parse simple string command: {}", cmd);
}

fn parse_array(buffer: &[u8]) {
    let data = std::str::from_utf8(buffer).expect("unable to read from buffer");
    let filtered_data: Vec<&str> = data.trim_end_matches("\r\n").split("\r\n").collect();
    println!("parse array string: {:?}", filtered_data);
}

fn parse_bulk_string(buffer: &[u8]) {
    let data = std::str::from_utf8(buffer).expect("unable to read from buffer");
    let filtered_data: Vec<&str> = data.trim_end_matches("\r\n").split("\r\n").collect();
    println!("parse bulk string: {:?}", filtered_data);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_array_parse() {
        let data = "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        parse_data_new(data.as_bytes());
    }

    #[test]
    fn test_simple_string_parse() {
        let data = "+1\r\n$4\r\nping\r\n";
        parse_data_new(data.as_bytes());
    }

    #[test]
    fn test_bulk_string_parse() {
        let data = "$6\r\nGET\r\n$5\r\nmykey\r\n";
        parse_data_new(data.as_bytes());
    }
}
