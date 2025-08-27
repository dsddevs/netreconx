use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Certificates {
    pub name_value: String
}

