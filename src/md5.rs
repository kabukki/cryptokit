use md5::Digest;
use crate::ActionResult;

pub fn hash (input: &String) -> ActionResult<String> {
    let mut hash = md5::Md5::new();
    hash.update(input.as_bytes());
    Ok(data_encoding::HEXLOWER_PERMISSIVE.encode(&hash.finalize()))
}

#[cfg(test)]
mod test {
    #[test]
    fn hash () {
        let cases = vec![
            ("", "d41d8cd98f00b204e9800998ecf8427e"),
            ("H", "c1d9f50f86825a1a2302ec2449c17196"),
            ("He", "a64cf5823262686e1a28b2245be34ce0"),
            ("Hel", "6b6e667a40e816c4da7bb4ab64cbb82b"),
            ("Hell", "1824e8e0307cbfdd1993511ab040075c"),
            ("Hello", "8b1a9953c4611296a827abf8c47804d7"),
            ("Hello ", "d1a7fb5eab1c16cb4f7cf341cf188c3d"),
            ("Hello W", "1e672754f0281a456e33680560954f0f"),
            ("Hello Wo", "d9bba15df6f560252129966ccd8b8922"),
            ("Hello Wor", "047cbe47e38cc43d24299f37d2deaac4"),
            ("Hello Worl", "8d23523511b7337dcd5ee9f7739d81cf"),
            ("Hello World", "b10a8db164e0754105b7a99be72e3fe5"),
        ];

        for case in cases {
            assert_eq!(
                super::hash(&case.0.to_string()),
                Ok(case.1.to_string()),
            );
        }
    }
}
