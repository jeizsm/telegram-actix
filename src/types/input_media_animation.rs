use super::*;

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Serialize, Deserialize, Debug)]
pub struct InputMediaAnimation {
    /// Type of the result, must be animation
    #[serde(rename = "type")]
    pub ty: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
    pub media: String,
    /// Optional. Thumbnail of the file sent. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 90. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    pub thumb: Option<Thumb>,
    /// Optional. Caption of the animation to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Optional. Animation width
    pub width: Option<Integer>,
    /// Optional. Animation height
    pub height: Option<Integer>,
    /// Optional. Animation duration
    pub duration: Option<Integer>,
}