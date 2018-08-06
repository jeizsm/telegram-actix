use super::super::types::*;

/// Use this method to send point on the map. On success, the sent Message is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendLocation {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<ChatId>,
    /// Latitude of the location
    pub latitude: Option<Float>,
    /// Longitude of the location
    pub longitude: Option<Float>,
    /// Period in seconds for which the location will be updated (see Live Locations, should be between 60 and 86400.
    pub live_period: Option<Integer>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}