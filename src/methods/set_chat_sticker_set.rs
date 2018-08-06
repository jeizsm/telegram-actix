use super::super::types::*;

/// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: Option<ChatId>,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: Option<String>,
}