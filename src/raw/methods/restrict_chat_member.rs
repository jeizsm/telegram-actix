use crate::types::*;

/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate admin rights. Pass True for all boolean parameters to lift restrictions from a user. Returns True on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct RestrictChatMember {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub(crate) chat_id: ChatIdOrUsername,
    /// Unique identifier of the target user
    pub(crate) user_id: Integer,
    /// Date when restrictions will be lifted for the user, unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) until_date: Option<Integer>,
    /// Pass True, if the user can send text messages, contacts, locations and venues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_send_messages: Option<bool>,
    /// Pass True, if the user can send audios, documents, photos, videos, video notes and voice notes, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_send_media_messages: Option<bool>,
    /// Pass True, if the user can send animations, games, stickers and use inline bots, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_send_other_messages: Option<bool>,
    /// Pass True, if the user may add web page previews to their messages, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_add_web_page_previews: Option<bool>,
}