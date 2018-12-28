use crate::types::*;

/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct InlineQueryResultPhoto {
    /// Type of the result, must be photo
    #[serde(rename = "type")]
    type_: String,
    /// Unique identifier for this result, 1-64 bytes
    id: String,
    /// A valid URL of the photo. Photo must be in jpeg format. Photo size must not exceed 5MB
    photo_url: String,
    /// URL of the thumbnail for the photo
    thumb_url: String,
    /// Width of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<Integer>,
    /// Height of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<Integer>,
    /// Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// Caption of the photo to be sent, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}
