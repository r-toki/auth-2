use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    encoded: String,
    name: String,
    content_type: String,
}
