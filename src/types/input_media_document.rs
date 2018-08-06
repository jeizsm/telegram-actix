use super::*;

/// Represents a general file to be sent.
#[derive(Serialize, Deserialize, Debug)]
pub struct InputMediaDocument {
    ty: String,
    media: String,
    thumb: Option<Thumb>,
    caption: Option<String>,
    parse_mode: Option<String>,
}