use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub encoded: String,
    pub name: String,
    pub content_type: String,
}
