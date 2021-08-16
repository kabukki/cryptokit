use hackit::uu;

#[test]
fn encode () {
    // assert_eq!(uu::encode(""), "");
    assert_eq!(uu::encode("H".as_bytes()), "!2```");
    assert_eq!(uu::encode("He".as_bytes()), "\"2&4`");
    assert_eq!(uu::encode("Hel".as_bytes()), "#2&5L");
    assert_eq!(uu::encode("Hell".as_bytes()), "$2&5L;```");
    assert_eq!(uu::encode("Hello".as_bytes()), "%2&5L;&\\`");
    assert_eq!(uu::encode("Hello ".as_bytes()), "&2&5L;&\\@");
    assert_eq!(uu::encode("Hello W".as_bytes()), "'2&5L;&\\@5P``");
    assert_eq!(uu::encode("Hello Wo".as_bytes()), "(2&5L;&\\@5V\\`");
    assert_eq!(uu::encode("Hello Wor".as_bytes()), ")2&5L;&\\@5V]R");
    assert_eq!(uu::encode("Hello Worl".as_bytes()), "*2&5L;&\\@5V]R;```");
    assert_eq!(uu::encode("Hello World".as_bytes()), "+2&5L;&\\@5V]R;&0`");
}

#[test]
fn decode () {
    // assert_eq!(uu::decode(""), "");
    assert_eq!(uu::decode("!2```"), "H");
    assert_eq!(uu::decode("\"2&4`"), "He");
    assert_eq!(uu::decode("#2&5L"), "Hel");
    assert_eq!(uu::decode("$2&5L;```"), "Hell");
    assert_eq!(uu::decode("%2&5L;&\\`"), "Hello");
    assert_eq!(uu::decode("&2&5L;&\\@"), "Hello ");
    assert_eq!(uu::decode("'2&5L;&\\@5P``"), "Hello W");
    assert_eq!(uu::decode("(2&5L;&\\@5V\\`"), "Hello Wo");
    assert_eq!(uu::decode(")2&5L;&\\@5V]R"), "Hello Wor");
    assert_eq!(uu::decode("*2&5L;&\\@5V]R;```"), "Hello Worl");
    assert_eq!(uu::decode("+2&5L;&\\@5V]R;&0`"), "Hello World");
}

#[test]
fn encode_value () {
    for (index, &character) in uu::CHARSET.iter().enumerate() {
        assert_eq!(uu::encode_value(index as u8), character);
    }
}

#[test]
fn decode_value () {
    for (index, &character) in uu::CHARSET.iter().enumerate() {
        assert_eq!(uu::decode_value(character), index as u8);
    }
}
