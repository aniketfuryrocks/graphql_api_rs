use async_graphql::Context;
use crate::jwt::Jwt;

use crate::api::app_state::AppState;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn query<'a>(&self, ctx: &Context<'a>) -> anyhow::Result<bool> {
        let _state = ctx.data::<AppState>().unwrap();
        let _jwt = ctx.data::<Jwt>().unwrap();

        todo!("")
    }
}
