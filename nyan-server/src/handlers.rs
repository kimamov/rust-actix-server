use crate::db;
use crate::models::{Status, User, SearchParams, Mail};
use crate::multi_part_handler::split_payload;

use actix_files as fs;
use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::{
    web, HttpRequest, HttpResponse, Responder,
    Result,
};
/* use actix_files::NamedFile;
use std::path::PathBuf; */
use std::collections::HashMap;
use actix_identity::{Identity};
use deadpool_postgres::{Client, Pool};
use std::borrow::BorrowMut;
use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};





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

            id.remember(user_name.name.to_owned());

            HttpResponse::Ok()

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

pub async fn send_mail(params: web::Form<Mail>)->impl Responder{
    /* for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    } */
    let user=std::env::var("MAIL.USER").expect("MAIL.USER must be set in .env");
    let password=std::env::var("MAIL.PASSWORD").expect("MAIL.PASSWORD must be set in .env");
    
    let email = Email::builder()
        .to("kantemir.imam@gmail.com")
        .from(params.email.to_string())
        .subject("subject")
        .html("<h1>Hi there</h1>")
        .text(params.message.to_string())
        .build()
        .unwrap();

    let creds = Credentials::new(
        user,
        password,
    );

    // Open connection to gmail
    let mut mailer = SmtpClient::new_simple("smtp.ionos.de")
        .unwrap()
        .credentials(creds)
        .transport();

    // Send the email
    let result = mailer.send(email.into());

   


    if result.is_ok() {
        /* println!("Could not send email: {:?}", params.email); */
        HttpResponse::Ok().json(Status {
            status: String::from("succesfully send mail")
        })
    } else {
        /* println!("Could not send email: {:?}", result); */
        HttpResponse::InternalServerError().json(Status {
            status: String::from("could not send mail! :(")
        })
    }
}

