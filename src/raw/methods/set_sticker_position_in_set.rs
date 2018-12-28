use crate::types::*;

/// Use this method to move a sticker in a set created by the bot to a specific position . Returns True on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[new(vis = "pub")]
#[set(vis = "pub", optional)]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    sticker: String,
    /// New sticker position in the set, zero-based
    position: Integer,
}
