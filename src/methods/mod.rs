mod delete_webhook;
mod edit_message_caption;
mod edit_message_text;
mod get_me;
mod get_webhook_info;
mod optimized_get_updates;
mod send_animation;
mod send_audio;
mod send_document;
mod send_message;
mod send_photo;
mod send_video;
mod send_voice;
mod set_webhook;

pub use self::delete_webhook::DeleteWebhook;
pub use self::edit_message_caption::EditMessageCaption;
pub use self::edit_message_text::EditMessageText;
pub use self::get_me::GetMe;
pub use self::get_webhook_info::GetWebhookInfo;
pub use self::optimized_get_updates::OptimizedGetUpdates;
pub use self::send_animation::SendAnimation;
pub use self::send_audio::SendAudio;
pub use self::send_document::SendDocument;
pub use self::send_message::SendMessage;
pub use self::send_photo::SendPhoto;
pub use self::send_video::SendVideo;
pub use self::send_voice::SendVoice;
pub use self::set_webhook::SetWebhook;
pub use crate::raw::methods::*;
