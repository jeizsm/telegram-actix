use super::super::types::*;

/// Use this method to get information about a member of a chat. Returns a ChatMember object on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: Option<ChatId>,
    /// Unique identifier of the target user
    pub user_id: Option<Integer>,
}