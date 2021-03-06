use crate::types::*;

/// Use this method to create new sticker set owned by a user. The bot will be able to edit the created sticker set. Returns True on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct CreateNewStickerSet {
    /// User identifier of created sticker set owner
    pub(crate) user_id: Integer,
    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals). Can contain only english letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in “_by_<bot username>”. <bot_username> is case insensitive. 1-64 characters.
    pub(crate) name: String,
    /// Sticker set title, 1-64 characters
    pub(crate) title: String,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub(crate) png_sticker: InputFileOrString,
    /// One or more emoji corresponding to the sticker
    pub(crate) emojis: String,
    /// Pass True, if a set of mask stickers should be created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) contains_masks: Option<bool>,
    /// A JSON-serialized object for position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mask_position: Option<MaskPosition>,
}