use crate::db;
use crate::models::Status;
use actix_files as fs;
use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::{
    web,
    web::{Data, Form},
    Error, HttpRequest, HttpResponse, Responder, Result,
};
use async_std::prelude::*;
use deadpool_postgres::{Client, Pool};
use futures::{StreamExt, TryStreamExt};
use bytes::Bytes;


pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

pub async fn get_projects(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_projects(&client).await;

    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().into(),
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

pub async fn project_form(req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../public/project_form.html")))
}

pub async fn create_project() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "done".to_string(),
    })
}

pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("./examples/{}", filename);
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}



pub fn upload_form() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/api/upload" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}
