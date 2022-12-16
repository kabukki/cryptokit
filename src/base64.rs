use crate::ActionResult;

pub fn encode (input: String) -> ActionResult<String> {
    Ok(data_encoding::BASE64.encode(input.as_ref()))
}

pub fn decode (input: String) -> ActionResult<String> {
    let bytes = data_encoding::BASE64.decode(&input.as_bytes()).map_err(|err| err.to_string())?;
    String::from_utf8(bytes).map_err(|err| err.to_string())
}

#[cfg(test)]
mod test {
    fn cases () -> Vec<(&'static str, &'static str)> {
        vec![
            ("", ""),
            ("H", "SA=="),
            ("He", "SGU="),
            ("Hel", "SGVs"),
            ("Hell", "SGVsbA=="),
            ("Hello", "SGVsbG8="),
            ("Hello ", "SGVsbG8g"),
            ("Hello W", "SGVsbG8gVw=="),
            ("Hello Wo", "SGVsbG8gV28="),
            ("Hello Wor", "SGVsbG8gV29y"),
            ("Hello Worl", "SGVsbG8gV29ybA=="),
            ("Hello World", "SGVsbG8gV29ybGQ="),
        ]
    }

    #[test]
    fn encode() {
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
