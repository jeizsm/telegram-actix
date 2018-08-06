use super::*;

/// This object represents information about an order.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInfo {
    /// Optional. User name
    pub name: Option<String>,
    /// Optional. User's phone number
    pub phone_number: Option<String>,
    /// Optional. User email
    pub email: Option<String>,
    /// Optional. User shipping address
    pub shipping_address: Option<ShippingAddress>,
}