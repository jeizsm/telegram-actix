use super::*;

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    /// Unique identifier for this file
    pub file_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: Integer,
    /// Optional. Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,
    /// Optional. Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<Integer>,
    /// Optional. Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>,
}