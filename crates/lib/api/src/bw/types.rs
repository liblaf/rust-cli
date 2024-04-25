use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Item {
    Login(ItemLogin),
    SecureNote(ItemSecureNote),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ItemLogin {
    pub name: String,
    pub notes: Option<String>,
    pub login: ItemLoginLogin,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ItemLoginLogin {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSecureNote {
    pub name: String,
    pub notes: Option<String>,
    pub secure_note: Value,
}
