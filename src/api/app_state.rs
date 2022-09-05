use crate::collections;

pub struct AppState {
    pub user_collection: mongodb::Collection<collections::Sample>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let user_collection = {
            let client = mongodb::Client::with_uri_str(crate::MONGO_ADDR).await?;
            let db = client.database(crate::MONGO_DB_NAME);
            db.collection(collections::sample::COLLECTION_NAME)
        };

        Ok(Self { user_collection })
    }
}
