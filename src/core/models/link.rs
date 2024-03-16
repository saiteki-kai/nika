use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub text: String,
    pub base_url: String,
}
