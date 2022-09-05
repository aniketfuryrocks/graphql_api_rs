use crate::api;
use crate::jwt::Claims;
use actix_web::{post, web, App, HttpServer};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

/// entry point for graph_ql requests
#[post("/")]
async fn index(
    schema: web::Data<api::schema::UserSchema>,
    req: GraphQLRequest,
    auth: BearerAuth,
) -> GraphQLResponse {
    let jwt = Claims::decode(auth.token()).expect("invalid jwt");
    schema.execute(req.into_inner().data(jwt)).await.into()
}

#[allow(clippy::let_and_return)]
pub async fn start_server() -> anyhow::Result<()> {
    // init the schema
    let schema = web::Data::new(api::schema::build_schema().await?);

    eprintln!("Starting aserver at {}", crate::API_ADDR);

    HttpServer::new(move || {
        let app = App::new().app_data(schema.clone()).service(index);

        #[cfg(feature = "cors")]
        let app = {
            use actix_web::http;
            eprintln!("Cors enabled");
            app.wrap(
                actix_cors::Cors::default()
                    .allowed_origin_fn(|_, _| true)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
        };

        app
    })
    .bind(crate::API_ADDR)?
    .run()
    .await?;

    Ok(())
}
