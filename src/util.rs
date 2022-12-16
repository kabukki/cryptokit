use crate::ActionResult;

pub fn strlen (input: String) -> ActionResult<usize> {
    Ok(input.len())
}
