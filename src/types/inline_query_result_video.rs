use super::*;

/// 
/// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you must replace its content using input_message_content.
/// 
/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVideo {
    /// Type of the result, must be video
    #[serde(rename = "type")]
    pub ty: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// Mime type of the content of video url, “text/html” or “video/mp4”
    pub mime_type: String,
    /// URL of the thumbnail (jpeg only) for the video
    pub thumb_url: String,
    /// Title for the result
    pub title: String,
    /// Optional. Caption of the video to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Optional. Video width
    pub video_width: Option<Integer>,
    /// Optional. Video height
    pub video_height: Option<Integer>,
    /// Optional. Video duration in seconds
    pub video_duration: Option<Integer>,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video. This field is required if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    pub input_message_content: Option<InputMessageContent>,
}