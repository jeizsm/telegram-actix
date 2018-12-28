use crate::types::*;

/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct PassportElementErrorSelfie {
    /// Error source, must be selfie
    source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    #[serde(rename = "type")]
    type_: String,
    /// Base64-encoded hash of the file with the selfie
    file_hash: String,
    /// Error message
    message: String,
}
