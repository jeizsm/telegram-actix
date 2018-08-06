use super::*;

/// This object represents a message.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message_id: Integer,
    from: Option<User>,
    date: Integer,
    chat: Chat,
    forward_from: Option<User>,
    forward_from_chat: Option<Chat>,
    forward_from_message_id: Option<Integer>,
    forward_signature: Option<String>,
    forward_date: Option<Integer>,
    reply_to_message: Box<Option<Message>>,
    edit_date: Option<Integer>,
    media_group_id: Option<String>,
    author_signature: Option<String>,
    text: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    caption_entities: Option<Vec<MessageEntity>>,
    audio: Option<Audio>,
    document: Option<Document>,
    animation: Option<Animation>,
    game: Option<Game>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Sticker>,
    video: Option<Video>,
    voice: Option<Voice>,
    video_note: Option<VideoNote>,
    caption: Option<String>,
    contact: Option<Contact>,
    location: Option<Location>,
    venue: Option<Venue>,
    new_chat_members: Option<Vec<User>>,
    left_chat_member: Option<User>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<Vec<PhotoSize>>,
    delete_chat_photo: Option<True>,
    group_chat_created: Option<True>,
    supergroup_chat_created: Option<True>,
    channel_chat_created: Option<True>,
    migrate_to_chat_id: Option<Integer>,
    migrate_from_chat_id: Option<Integer>,
    pinned_message: Box<Option<Message>>,
    invoice: Option<Invoice>,
    successful_payment: Option<SuccessfulPayment>,
    connected_website: Option<String>,
    passport_data: Option<PassportData>,
}