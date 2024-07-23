use actix_cors::Cors;
use actix_web::{
    get, guard,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use async_graphql::{http::GraphiQLSource, BatchResponse, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use std::sync::Arc;

use crate::gql::Resolver;
mod auth;
mod db;
mod gql;
mod model;

type AppSchema = Schema<gql::Query, gql::Mutation, EmptySubscription>;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

async fn index(
    schema: web::Data<AppSchema>,
    http_req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut query = gql_req.into_inner();
    let r = http_req.app_data::<web::Data<Arc<Resolver>>>();
    if let None = r {
        return GraphQLResponse::from(BatchResponse::Single(async_graphql::Response::from_errors(
            vec![async_graphql::ServerError::new("Resolver not found", None)],
        )));
    }
    let r = r.unwrap().as_ref();

    let token = auth::token_from_header(&http_req);
    match auth::resolve_token(token, r.clone()).await {
        Err(msg) => {
            return GraphQLResponse::from(BatchResponse::Single(
                async_graphql::Response::from_errors(vec![async_graphql::ServerError::new(
                    format!("Error resolving token, {}", msg),
                    None,
                )]),
            ))
        }
        Ok(result) => {
            if let Some(account) = result {
                query = query.data(account);
            }
            schema.execute(query).await.into()
        }
    }
}

async fn index_graphiql() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/")
                .finish(),
        ))
}

async fn index_ws(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("GraphiQL IDE: http://localhost:8000");
    let r = Resolver::new().await;
    let ar = Arc::new(r);
    let schema = Schema::build(gql::Query, gql::Mutation, EmptySubscription)
        .data(ar.clone())
        .finish();

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(ar.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(greet)
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
