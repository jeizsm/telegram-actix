use crate::types::*;

/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct PassportElementErrorTranslationFiles {
    /// Error source, must be translation_files
    pub(crate) source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// List of base64-encoded file hashes
    pub(crate) file_hashes: Vec<String>,
    /// Error message
    pub(crate) message: String,
}