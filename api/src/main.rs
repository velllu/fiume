use errors::ApiError;

use crate::parsing::Settings;

mod errors;
mod parsing;

fn main() -> Result<(), ApiError> {
    let settings: Settings = serde_json::from_str(include_str!("../../settings.json"))?;
    Ok(())
}
