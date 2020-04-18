use crate::models::Status;
use crate::db;

use actix_files as fs;
/* use actix_session::{CookieSession, Session}; */
/* use actix_utils::mpsc; */
use actix_web::http::{/* header, Method,  */StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder,
    Result,
};
use deadpool_postgres::{Pool, Client};
use std::path::PathBuf;





pub async fn status()->impl Responder{
    web::HttpResponse::Ok()
        .json(Status {status: "UP".to_string() })
}

pub async fn get_projects(db_pool: web::Data<Pool>)->impl Responder{
    let client: Client=
        db_pool.get().await.expect("Error connecting to the database");

    let result=db::get_projects(&client).await;

    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_)=>HttpResponse::InternalServerError().into()
    }
}

/* pub async fn frontend()->Result<fs::NamedFile, actix_web::error::Error>{
    let path: PathBuf=PathBuf::from("public/index.html");
    
    Ok(fs::NamedFile::open(path)?)
} */

/* #[get("/welcome")] */
pub async fn index(req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../public/index.html")))
}

pub async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("public/index.html")?.set_status_code(StatusCode::NOT_FOUND))
}