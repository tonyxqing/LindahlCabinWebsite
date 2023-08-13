use actix_web::{guard, web, web::Data, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Enum, Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
pub type Storage = Arc<Mutex<Resolver>>;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Role {
    Owner,
    Member,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub role: Role,
}

#[Object]
impl User {
    async fn name(&self) -> &str {
        &self.name
    }
    async fn email(&self) -> &str {
        &self.email
    }
    async fn phone(&self) -> &str {
        &self.phone
    }
    async fn role(&self) -> &Role {
        &self.role
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            role: Role::Member,
        }
    }
}

pub struct Resolver {
    pub db: Database,
}

impl Resolver {
    pub async fn new() -> Self {
        let client_options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .expect("Error occured with mongodb client options");
        let client = Client::with_options(client_options).expect("Can't connect to mongodb client");
        Self {
            db: client.database("LFC"),
        }
    }

    pub async fn from_context(ctx: &Context<'_>) -> Storage {
        ctx.data::<Storage>()
            .expect("Could not find resolver in context")
            .clone()
    }

    pub fn get_users(&self) -> String {
        "Memer".to_string()
    }

    pub fn add_user(&self, user: User) -> Result<User, String> {
        let collection = self.db.collection::<User>("Users");
        collection.insert_one(user.clone(), None);
        Ok(user)
    }
}
pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        let res = a + b;
        println!("{}", res);
        res
    }
    async fn get_users(&self, ctx: &Context<'_>) -> String {
        let r = ctx
            .data::<Storage>()
            .expect("Error retrieving context")
            .clone();
        let r = r.lock().unwrap();
        r.get_users()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn add_member(
        &self,
        ctx: &Context<'_>,
        name: String,
        email: String,
        phone: String,
        role: Role,
    ) -> Result<User, String> {
        let r = Resolver::from_context(ctx).await;
        let r = r.lock().unwrap();
        let user = User {
            name,
            email,
            phone,
            role,
        };
        r.add_user(user.clone())
    }
}

type AppSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
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
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(Storage::new(Mutex::new(Resolver::new().await)))
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
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
