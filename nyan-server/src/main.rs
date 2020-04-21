mod config;
mod models;
mod handlers;
mod db;

use actix_web::{HttpServer, App, web};
use tokio_postgres::NoTls;
use std::io;
use dotenv::dotenv;

/* use crate::models::Status; */
use crate::handlers::*;
/* use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt}; */


#[actix_rt::main]
async fn main()->io::Result<()> {

    dotenv().ok();

    let config=crate::config::Config::from_env().unwrap();

    let pool=config.pg.create_pool(NoTls).unwrap();

    /* let form = Form::new()
    .field("Hey", Field::text())
    .field(
        "Hi",
        Field::map()
            .field("One", Field::int())
            .field("Two", Field::float())
            .finalize(),
    )
    .field("files", Field::array(Field::file(FileName))); */

    /* println!("{:?}", form); */

    println!("Hello, world!");
    HttpServer::new(move|| {
        App::new()
            .data(pool.clone())
            /* .data(form.clone()) */
            .route("/api/", web::get().to(status))
            .route("/api/projects", web::get().to(get_projects))
            .route("/api/projects", web::post().to(create_project))
            /* .route("/api/upload", web::post().to(upload)) */
            .route("/api/upload", web::post().to(save_file))
            .route("/api/upload", web::get().to(upload_form))
            .route("/api/projectform",web::get().to(project_form))
            .route("/test",web::get().to(p404))
            .route("/",web::get().to(index))
            
    })
    .bind(format!("{}:{}", config.server.host ,config.server.port))?
    .run()
    .await
}
