use async_graphql::Context;
use crate::jwt::Jwt;

use crate::api::app_state::AppState;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn mutate(&self, ctx: &Context<'_>) -> anyhow::Result<bool> {
        let _state = ctx.data::<AppState>().unwrap();
        let _jwt = ctx.data::<Jwt>().unwrap();

        todo!("")
    }
}
