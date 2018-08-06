use super::super::types::*;

/// Use this method to forward messages of any kind. On success, the sent Message is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<ChatId>,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    pub from_chat_id: Option<FromChatId>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: Option<Integer>,
}