use actix_web::{get, guard, http::header, web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer, Responder};
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


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

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
    println!("Building Schema");
    let schema = Schema::build(gql::Query, gql::Mutation, EmptySubscription)
    .data(ar.clone())
    .finish();

    HttpServer::new(move || {
        println!("Starting Http Server");
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
            .service(greet)
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
