#[derive(Debug)]
pub enum ApiError {
    CouldntParseJson,
}

impl From<serde_json::Error> for ApiError {
    fn from(_value: serde_json::Error) -> Self {
        Self::CouldntParseJson
    }
}
