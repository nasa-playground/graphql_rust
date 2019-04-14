#[macro_use]
extern crate juniper;

mod contriview;
mod schema;

use crate::schema::{create_schema, Schema};
use actix_web::{
    http, middleware::cors::Cors, middleware::Logger, web, App, Error, HttpResponse, HttpServer,
};
use futures::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    schema: web::Data<Arc<Schema>>,
    request: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    log::info!("graphql");
    web::block(move || {
        let res = request.execute(&schema, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|res| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(res))
    })
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "graphql_example");
    env_logger::init();

    log::info!("server lunch!, localhost::8080");

    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
