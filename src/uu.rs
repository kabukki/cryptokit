use crate::ActionResult;

fn get_encoding (padding: bool) -> data_encoding::Encoding {
    let mut spec = data_encoding::Specification::new();
    spec.symbols.push_str("`!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_");
    spec.padding = if padding { Some(' ') } else { None };
    spec.encoding().unwrap()
}

pub fn encode (input: String) -> ActionResult<String> {
    let encoding = get_encoding(true);
    let mut output = encoding.encode(input.as_bytes());
    output.insert(0, (input.len() as u8 + 32) as char);

    Ok(output.replace(' ', "`"))
}

pub fn decode (input: String) -> ActionResult<String> {
    let length = (input.as_bytes()[0] - 32) as usize;
    let encoding = get_encoding(false);
    
    let mut bytes = encoding.decode(&input.as_bytes()[1..]).map_err(|err| err.to_string())?;
    bytes.truncate(length);
    String::from_utf8(bytes).map_err(|err| err.to_string())
}

#[cfg(test)]
mod test {
    fn cases () -> Vec<(&'static str, &'static str)> {
        vec![
            ("", "`"),
            ("H", "!2```"),
            ("He", "\"2&4`"),
            ("Hel", "#2&5L"),
            ("Hell", "$2&5L;```"),
            ("Hello", "%2&5L;&\\`"),
            ("Hello ", "&2&5L;&\\@"),
            ("Hello W", "'2&5L;&\\@5P``"),
            ("Hello Wo", "(2&5L;&\\@5V\\`"),
            ("Hello Wor", ")2&5L;&\\@5V]R"),
            ("Hello Worl", "*2&5L;&\\@5V]R;```"),
            ("Hello World", "+2&5L;&\\@5V]R;&0`"),
            ("Oh!", "#3V@A"),
            ("Cat", "#0V%T"),
        ]
    }

    #[test]
    fn encode () {
        for case in cases() {
            assert_eq!(
                super::encode(case.0.to_string()),
                Ok(case.1.to_string()),
            );
        }
    }

    #[test]
    fn decode () {
        for case in cases() {
            assert_eq!(
                super::decode(case.1.to_string()),
                Ok(case.0.to_string()),
            );
        }
    }
}
