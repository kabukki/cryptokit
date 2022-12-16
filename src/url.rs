use crate::ActionResult;

pub fn encode (input: String) -> ActionResult<String> {
    Ok(urlencoding::encode(&input).into_owned())
}

pub fn decode (input: String) -> ActionResult<String> {
    urlencoding::decode(&input)
        .map_err(|err| err.to_string())
        .map(|output| output.into_owned())
}
