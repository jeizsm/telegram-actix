use super::super::types::*;

/// Use this method to send invoices. On success, the sent Message is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendInvoice {
    /// Unique identifier for the target private chat
    pub chat_id: Option<Integer>,
    /// Product name, 1-32 characters
    pub title: Option<String>,
    /// Product description, 1-255 characters
    pub description: Option<String>,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub payload: Option<String>,
    /// Payments provider token, obtained via Botfather
    pub provider_token: Option<String>,
    /// Unique deep-linking parameter that can be used to generate this invoice when used as a start parameter
    pub start_parameter: Option<String>,
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub currency: Option<String>,
    /// Price breakdown, a list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub prices: Option<Vec<LabeledPrice>>,
    /// JSON-encoded data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
    pub photo_url: Option<String>,
    /// Photo size
    pub photo_size: Option<Integer>,
    /// Photo width
    pub photo_width: Option<Integer>,
    /// Photo height
    pub photo_height: Option<Integer>,
    /// Pass True, if you require the user's full name to complete the order
    pub need_name: Option<bool>,
    /// Pass True, if you require the user's phone number to complete the order
    pub need_phone_number: Option<bool>,
    /// Pass True, if you require the user's email address to complete the order
    pub need_email: Option<bool>,
    /// Pass True, if you require the user's shipping address to complete the order
    pub need_shipping_address: Option<bool>,
    /// Pass True, if user's phone number should be sent to provider
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass True, if user's email address should be sent to provider
    pub send_email_to_provider: Option<bool>,
    /// Pass True, if the final price depends on the shipping method
    pub is_flexible: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<Integer>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown. If not empty, the first button must be a Pay button.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}