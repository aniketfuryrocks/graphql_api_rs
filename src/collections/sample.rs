use serde::{Deserialize, Serialize};

pub const COLLECTION_NAME: &str = "collection_name";

/// mongo db collection
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sample {
    sample: String,
}
