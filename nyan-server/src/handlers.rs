use crate::db;
use crate::models::{Mail, SearchParams, Status, User};
use crate::multi_part_handler::split_payload;

use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use deadpool_postgres::{Client, Pool};
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::Email;
use std::borrow::BorrowMut;

fn redirect_to_log_in() -> HttpResponse {
    HttpResponse::TemporaryRedirect()
        .header("location", "/api/login")
        .finish()
}
fn redirect_to_home() -> HttpResponse {
    HttpResponse::TemporaryRedirect()
        .header("location", "/api/")
        .finish()
}

pub async fn status(id: Identity) -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: id.identity().unwrap_or_else(|| "guest_user".to_owned()),
    })
}

pub async fn get_projects(
    db_pool: web::Data<Pool>,
    query: web::Query<SearchParams>,
) -> impl Responder {
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

pub async fn project_form(id: Identity, _req: HttpRequest) -> impl Responder {
    // redirect to login if not logged in otherwise serve create project form
    match id.identity() {
        Some(_) => HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../public/project_form.html")),
        None => redirect_to_log_in(),
    }
}

pub async fn create_admin(db_pool: &Pool) {
    let user = std::env::var("ADMIN.NAME").expect("ADMIN.NAME must be set in .env");
    let password = std::env::var("ADMIN.PASSWORD").expect("ADMIN.PASSWORD must be set in .env");
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    // create hash for the password
    let hashed_password = hash(password.to_string(), DEFAULT_COST).unwrap();

    let user: User = User {
        id: None,
        name: user.to_string(),
        password: hashed_password,
    };
    db::create_user(&client, user)
        .await
        .expect("Error creating admin");
}

pub async fn log_in_form(_req: HttpRequest) -> Result<HttpResponse> {
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../public/login.html")))
}

pub async fn create_project(
    id: Identity,
    mut payload: Multipart,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    match id.identity() {
        Some(_) => {
            let project = split_payload(payload.borrow_mut()).await;

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
        None => redirect_to_log_in(),
    }
}

pub async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../public/index.html")))
}

pub async fn log_in(
    params: web::Form<User>,
    db_pool: web::Data<Pool>,
    id: Identity,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    // create hash for the password

    let user = db::log_in(&client, params.name.to_string()).await;

    match user {
        Ok(user_data) => {
            // check if users password matches the newly hashed password
            let hashed_password = hash(params.password.to_string(), DEFAULT_COST).unwrap();

            let result = verify(user_data.password, &hashed_password);
            match result {
                Ok(_r) => {
                    id.remember(user_data.name.to_owned());
                    redirect_to_home()
                }
                // user found but password not matching error
                Err(_) => HttpResponse::NotFound()
                    .content_type("text/plain")
                    .body("could not find user with this combination of name and password"),
            }
        }
        // no user error
        Err(_) => HttpResponse::NotFound()
            .content_type("text/plain")
            .body("could not find user with this combination of name and password"),
    }
}

pub async fn log_out(id: Identity) -> impl Responder {
    id.forget();
    redirect_to_home()
}

/* pub async fn static_files(req: HttpRequest)->Result<NamedFile>{
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
} */

pub async fn send_mail(params: web::Form<Mail>) -> impl Responder {
    let user = std::env::var("RUSTMAIL.USER").expect("MAIL.USER must be set in .env");
    let password = std::env::var("RUSTMAIL.PASSWORD").expect("MAIL.PASSWORD must be set in .env");
    let email = Email::builder()
        .to("kantemir.imam@gmail.com")
        .from(params.email.to_string())
        .subject("subject")
        .html("<h1>Hi there</h1>")
        .text(params.message.to_string())
        .build()
        .unwrap();

    let creds = Credentials::new(user, password);

    // Open connection to gmail
    let mut mailer = SmtpClient::new_simple("smtp.ionos.de")
        .unwrap()
        .credentials(creds)
        .transport();

    // Send the email
    let result = mailer.send(email.into());

    if result.is_ok() {
        HttpResponse::Ok().json(Status {
            status: String::from("succesfully send mail"),
        })
    } else {
        HttpResponse::InternalServerError().json(Status {
            status: String::from("could not send mail! :("),
        })
    }
}
