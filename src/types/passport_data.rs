use super::*;

/// Contains information about Telegram Passport data shared with the bot by the user.
#[derive(Serialize, Deserialize, Debug)]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport elements that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}