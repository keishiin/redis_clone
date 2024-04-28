use core::panic;

#[derive(Clone, Debug)]
pub enum Value {
    Simple(String),
    Bulk(String),
    Array(Vec<Value>),
    Error(String),
}

impl Value {
    pub fn serialize(self) -> String {
        match self {
            Value::Simple(s) => format!("+{}\r\n", s),
            Value::Bulk(s) => format!("${}\r\n{}\r\n", s.chars().count(), s),
            Value::Error(s) => format!("-Error {}\r\n", s),
            _ => panic!("unable to serialize"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seriaze_simple_value() {
        let v = Value::Simple("test".to_string());
        assert_eq!(v.serialize(), "+test\r\n");
    }

    #[test]
    fn test_seriaze_bulk_value() {
        let v = Value::Bulk("key_value".to_string());
        assert_eq!(v.serialize(), "$9\r\nkey_value\r\n");
    }

    #[test]
    fn test_seriaze_error_value() {
        let v = Value::Error("Test Error".to_string());
        assert_eq!(v.serialize(), "-Error Test Error\r\n");
    }
}
