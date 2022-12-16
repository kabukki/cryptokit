use crate::ActionResult;

pub fn encode (input: String) -> ActionResult<String> {
    Ok(data_encoding::HEXLOWER_PERMISSIVE.encode(input.as_ref()))
}

pub fn decode (input: String) -> ActionResult<String> {
    let bytes = data_encoding::HEXLOWER_PERMISSIVE.decode(&input.as_bytes()).map_err(|err| err.to_string())?;
    String::from_utf8(bytes).map_err(|err| err.to_string())
}

#[cfg(test)]
mod test {
    fn cases () -> Vec<(&'static str, &'static str)> {
        vec![
            ("", ""),
            ("H", "48"),
            ("He", "4865"),
            ("Hel", "48656c"),
            ("Hell", "48656c6c"),
            ("Hello", "48656c6c6f"),
            ("Hello ", "48656c6c6f20"),
            ("Hello W", "48656c6c6f2057"),
            ("Hello Wo", "48656c6c6f20576f"),
            ("Hello Wor", "48656c6c6f20576f72"),
            ("Hello Worl", "48656c6c6f20576f726c"),
            ("Hello World", "48656c6c6f20576f726c64"),
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
