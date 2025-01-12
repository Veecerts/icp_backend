use actix_cors::Cors;
use actix_web::{
    get, http::header::HeaderMap, post, web, App, HttpRequest, HttpResponse, HttpServer,
};
use apps::users::utils::auth::decode_user_auth_token;
use async_graphql::{
    http::{graphiql_plugin_explorer, GraphiQLSource},
    Result,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use config::{
    database::connect_db,
    schema::{get_schema, AppSchema},
    settings::ENV,
};
use dotenv::dotenv;
use entity::entities::user;
use sea_orm::DatabaseConnection;
pub mod apps;
pub mod config;

#[get("/")]
async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .plugins(&[graphiql_plugin_explorer()])
                .endpoint("/")
                .finish(),
        )
}

async fn get_user_from_header(
    headers: &HeaderMap,
    db: &DatabaseConnection,
) -> Result<Option<user::Model>> {
    let token_str = headers
        .get("Authorization")
        .map(|value| Some(value.as_ref()));

    if let Some(token_str) = token_str {
        match token_str {
            Some(token) => {
                let token = decode_user_auth_token(String::from_utf8(token.to_vec())?, db).await?;
                Ok(token)
            }
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

#[post("/")]
async fn index(
    schema: web::Data<AppSchema>,
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let user = match get_user_from_header(req.headers(), &db).await {
        Ok(user) => user,
        Err(err) => {
            return async_graphql::Response::from_errors(vec![
                err.into_server_error(async_graphql::Pos { line: 0, column: 0 })
            ])
            .into()
        }
    };
    let request = gql_request.into_inner().data(user);
    schema.execute(request).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let env = ENV::init();
    let port = env.port;
    let addrs = env.addrs;
    let db_conn = connect_db().await.expect("Database connection failed");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    println!("Server running on http://{}:{}", &addrs, &port);
    HttpServer::new(move || {
        let allowed_origins = ENV::init().allowed_origns;
        let cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .allowed_origin_fn(move |origin, _req_head| {
                if let Ok(origin_str) = String::from_utf8(origin.as_bytes().to_vec()) {
                    allowed_origins
                        .split(",")
                        .any(|host| host == origin_str.as_str())
                } else {
                    false
                }
            });

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db_conn.clone()))
            .app_data(web::Data::new(get_schema(db_conn.clone())))
            .service(graphiql)
            .service(index)
    })
    .bind((addrs, port))?
    .run()
    .await
}
