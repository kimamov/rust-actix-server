use crate::db;
use crate::models::{Status, User, SearchParams};
use crate::multi_part_handler::split_payload;

use actix_files as fs;
use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::{
    web, HttpRequest, HttpResponse, Responder,
    Result,
};
use actix_files::NamedFile;
use std::path::PathBuf;
use actix_identity::{Identity};
use deadpool_postgres::{Client, Pool};
use std::borrow::BorrowMut;

pub async fn status(id: Identity) -> impl Responder {

    HttpResponse::Ok().json(Status {
        status: id.identity().unwrap_or_else(|| "guest_user".to_owned()),
    })
}

pub async fn get_projects(db_pool: web::Data<Pool>, query: web::Query<SearchParams>) -> impl Responder {    
    match query.limit {
        Some(data) =>  println!("{}", data),
        None => println!("nothing found :(")
    };
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_projects(&client, query.limit, query.offset).await;

    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn project_form(req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../public/project_form.html")))
}

pub async fn create_project(mut payload: Multipart, db_pool: web::Data<Pool>) -> impl Responder {
    let project = split_payload(payload.borrow_mut()).await;
    println!("bytes={:#?}", project);

    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::create_project(&client, project).await;

    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().into(),
    }

}

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


pub async fn log_in(db_pool: web::Data<Pool>, id: Identity)->impl Responder{
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    
    let user: User=User{
        id: None,
        name: Some(String::from("kantemir")),
        password: Some(String::from("test"))
    };
    let result=db::log_in(&client, user)
        .await;

    match result {
        Ok(user_name) =>{
            /* IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false)
                    .max_age(86400) // 1 day in seconds
            ); */
            id.remember(user_name.name.to_owned());

            HttpResponse::Ok()
                /* .cookie(
                    http::Cookie::build("user_token", "test")
                        .secure(false)
                        .max_age(86400) // 1 day in seconds
                        .finish()
                ) */
                .json(user_name)
               
                
        },
        Err(_) => HttpResponse::NotFound()
            .content_type("text/plain")
            .body("Not Found"),
    }
}

pub async fn log_out(id: Identity)->impl Responder{
    id.forget();
    HttpResponse::Ok().json(Status {
        status: id.identity().unwrap_or_else(|| "guest_user".to_owned()),
    })
}


/* pub async fn static_files(req: HttpRequest)->Result<NamedFile>{
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
} */