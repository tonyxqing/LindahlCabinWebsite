use actix_web::{guard, web, web::Data, App, HttpRequest, HttpResponse, HttpServer, http::header};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use actix_cors::Cors;
use std::sync::Arc;

use crate::gql::Resolver;
mod db;
mod gql;
mod model;
mod auth;

type AppSchema = Schema<gql::Query, gql::Mutation, EmptySubscription>;

async fn index(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    // let data = req.app_data::<web::Data<>>
    schema.execute(req.into_inner()).await.into()
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
            .wrap(Cors::default().allow_any_origin().send_wildcard())
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(ar.clone()))            
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
