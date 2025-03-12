pub trait TTInput {
    fn from_json(json: &serde_json::Value) -> Result<Self, Vec<crate::validation::Error>>
    where
        Self: Sized;
}

pub trait ToTTError {
    fn to_tt_error(&self) -> crate::validation::Error;
}

impl ToTTError for sqlx::Error {
    fn to_tt_error(&self) -> crate::validation::Error {
        crate::validation::Error::internal_error(self)
    }
}
