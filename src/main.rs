use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;
use std::fmt;

#[derive(Debug)]
enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(s) => write!(f, r#""{}""#, s),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                let mut first = true;
                for item in arr {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(obj) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in obj {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, r#""{}": {}"#, key, value)?;
                }
                write!(f, "}}")
            }
        }
    }
}

fn parse_value(chars: &mut Peekable<Chars>) -> Result<JsonValue, String> {
    match chars.peek() {
        Some(&'n') => parse_null(chars),
        Some(&'t') | Some(&'f') => parse_boolean(chars),
        Some(&'"') => parse_string(chars),
        Some(&('0'..='9') | '-') => parse_number(chars),
        Some(&'[') => parse_array(chars),
        Some(&'{') => parse_object(chars),
        Some(&' ') => {
            chars.next();
            parse_value(chars)
        }
        Some(_) => Err("Unexpected character".to_string()),
        None => Err("Unexpected end of input".to_string()),
    }
}

fn parse_null(chars: &mut Peekable<Chars>) -> Result<JsonValue, String> {
    let null_str: String = chars.by_ref().take(4).collect();
    if null_str == "null" {
        Ok(JsonValue::Null)
    } else {
        Err("Invalid null value".to_string())
    }
}

fn parse_boolean(chars: &mut Peekable<Chars>) -> Result<JsonValue, String> {
    let mut boolean_str = String::new();
    while let Some(&character) = chars.peek() {
        if character.is_alphabetic() {
            boolean_str.push(character);
            chars.next();
        } else {
            break;
        }
    }

    match boolean_str.as_str() {
        "true" => Ok(JsonValue::Boolean(true)),
        "false" => Ok(JsonValue::Boolean(false)),
        _ => Err("Invalid boolean value".to_string()),
    }
}

fn parse_string(chars: &mut Peekable<Chars>) -> Result<JsonValue, String> {
    let mut string_value = String::new();
    // Consume the opening '"'
    chars.next();

    while let Some(character) = chars.next() {
        if character == '"' {
            return Ok(JsonValue::String(string_value));
        }
        string_value.push(character);
    }

    Err("Unterminated string".to_string())
}

fn parse_number(chars: &mut Peekable<Chars>) -> Result<JsonValue, String> {
    let mut number_buffer = String::new();

    while let Some(&character) = chars.peek() {
        match character {
            '0'..='9' | '-' | '.' | 'e' | 'E' => {
                number_buffer.push(character);
                chars.next();
            }
            _ => break,
        }
    }

    match number_buffer.parse::<f64>() {
        Ok(number) => Ok(JsonValue::Number(number)),
        Err(_) => Err("Invalid number format".to_string()),
    }
}

fn parse_array(chars: &mut Peekable<Chars>) -> Result<JsonValue, String> {
    let mut array = Vec::new();

    // Consume the opening '['
    chars.next();

    while let Some(&character) = chars.peek() {
        match character {
            ']' => {
                // Consume the closing ']'
                chars.next();
                return Ok(JsonValue::Array(array));
            }
            ',' => {
                // Consume the comma and continue
                chars.next();
            }
            ' ' => {
                // Ignore whitespace
                chars.next();
            }
            _ => {
                match parse_value(chars) {
                    Ok(value) => array.push(value),
                    Err(err) => return Err(err),
                }
            }
        }
    }

    Err("Unterminated array".to_string())
}

fn parse_object(chars: &mut Peekable<Chars>) -> Result<JsonValue, String> {
    let mut object = HashMap::new();

    // Skip leading whitespace characters
    while let Some(&character) = chars.peek() {
        
        if character.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }

    // Check for the opening '{'
    match chars.next() {
        Some('{') => {}
        _ => return Err("Expected '{' to start an object".to_string()),
    }

    while let Some(&character) = chars.peek() {
        
        match character {
            '}' => {
                chars.next();
                return Ok(JsonValue::Object(object));
            }
            ',' => {
                chars.next();
            }
            ' ' => {
                chars.next();
            }
            '"' => {
                let key = parse_string(chars).unwrap().to_string(); // Convert key to string without double quotes
                println!("{}", key);
                match chars.next() {
                    Some(':') => {
                        let value = parse_value(chars)?;
                        object.insert(key, value);
                    }
                    _ => return Err("Expected ':' after key in object".to_string()),
                }
            }
            _ => return Err("Invalid character in object".to_string()),
        }
    }

    Err("Unterminated object".to_string())
}

fn main() {
    let json_str = r#"{"name": "Alice","age": 30,"isStudent": false,"hobbies": ["reading", "swimming", "coding"],"address": {"street": "123 Main St","city": "Wonderland","zip": "12345"}}"#;

    match parse_value(&mut json_str.chars().peekable()) {
        Ok(parsed_json) => {
            println!("{:?} jsp", parsed_json);
        }
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
        }
    }
}
