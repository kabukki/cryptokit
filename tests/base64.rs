use hackit::base64;

#[test]
fn encode () {
    assert_eq!(base64::encode(&[]), "");
    assert_eq!(base64::encode("H".as_bytes()), "SA==");
    assert_eq!(base64::encode("He".as_bytes()), "SGU=");
    assert_eq!(base64::encode("Hel".as_bytes()), "SGVs");
    assert_eq!(base64::encode("Hell".as_bytes()), "SGVsbA==");
    assert_eq!(base64::encode("Hello".as_bytes()), "SGVsbG8=");
    assert_eq!(base64::encode("Hello ".as_bytes()), "SGVsbG8g");
    assert_eq!(base64::encode("Hello W".as_bytes()), "SGVsbG8gVw==");
    assert_eq!(base64::encode("Hello Wo".as_bytes()), "SGVsbG8gV28=");
    assert_eq!(base64::encode("Hello Wor".as_bytes()), "SGVsbG8gV29y");
    assert_eq!(base64::encode("Hello Worl".as_bytes()), "SGVsbG8gV29ybA==");
    assert_eq!(base64::encode("Hello World".as_bytes()), "SGVsbG8gV29ybGQ=");
}

#[test]
fn decode () {
    assert_eq!(base64::decode(""), "");
    assert_eq!(base64::decode("SA=="), "H");
    assert_eq!(base64::decode("SGU="), "He");
    assert_eq!(base64::decode("SGVs"), "Hel");
    assert_eq!(base64::decode("SGVsbA=="), "Hell");
    assert_eq!(base64::decode("SGVsbG8="), "Hello");
    assert_eq!(base64::decode("SGVsbG8g"), "Hello ");
    assert_eq!(base64::decode("SGVsbG8gVw=="), "Hello W");
    assert_eq!(base64::decode("SGVsbG8gV28="), "Hello Wo");
    assert_eq!(base64::decode("SGVsbG8gV29y"), "Hello Wor");
    assert_eq!(base64::decode("SGVsbG8gV29ybA=="), "Hello Worl");
    assert_eq!(base64::decode("SGVsbG8gV29ybGQ="), "Hello World");
}

#[test]
fn encode_value () {
    for (index, &character) in base64::CHARSET.iter().enumerate() {
        assert_eq!(base64::encode_value(index as u8), character);
    }
}

#[test]
fn decode_value () {
    for (index, &character) in base64::CHARSET.iter().enumerate() {
        assert_eq!(base64::decode_value(character), index as u8);
    }
}
