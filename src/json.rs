use crate::ActionResult;
use serde_json::Value;

pub fn compact (input: String) -> ActionResult<String> {
    let json: Value = serde_json::from_str(&input).map_err(|err| err.to_string())?;
    serde_json::to_string(&json).map_err(|err| err.to_string())
}

pub fn pretty (input: String) -> ActionResult<String> {
    let json: Value = serde_json::from_str(&input).map_err(|err| err.to_string())?;
    serde_json::to_string_pretty(&json).map_err(|err| err.to_string())
}

pub fn test (input: String) -> ActionResult<()> {
    serde_json::from_str::<Value>(&input)
        .and(Ok(()))
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod test {
    #[test]
    fn test () {
        assert!(super::test("".to_string()).is_err());
        assert!(super::test("{}".to_string()).is_ok());
        assert!(super::test("[]".to_string()).is_ok());
        assert!(super::test("[]]".to_string()).is_err());
    }
}
