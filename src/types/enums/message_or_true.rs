use types::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MessageOrTrue {
    Message(Message),
    True(True),
}
