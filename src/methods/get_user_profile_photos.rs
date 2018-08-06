use super::super::types::*;

/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: Option<Integer>,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    pub offset: Option<Integer>,
    /// Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    pub limit: Option<Integer>,
}