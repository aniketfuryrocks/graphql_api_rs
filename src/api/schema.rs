use super::{app_state::AppState, mutation, query};
use async_graphql::{EmptySubscription, Schema};

pub type UserSchema = Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>;

pub async fn build_schema() -> anyhow::Result<UserSchema> {
    Ok(
        Schema::build(query::QueryRoot, mutation::MutationRoot, EmptySubscription)
            .data(AppState::new().await?)
            .finish(),
    )
}
