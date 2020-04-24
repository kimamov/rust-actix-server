mod config;
mod db;
mod handlers;
mod models;
mod multi_part_handler;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_session::{CookieSession, Session};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

/* use crate::models::Status; */
use crate::handlers::*;





#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Hello, world!");
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false)
                    .max_age(86400) // 1 day in seconds
            ))
            .data(pool.clone())
            
            .service(web::scope("/api/")
                .service(web::resource("/projects")
                    .route(web::get().to(get_projects))
                    .route(web::post().to(create_project)
                ))
                .service(web::resource("/projectform")
                    .route(web::get().to(project_form)
                ))
                .service(web::resource("/login")
                    .route(web::get().to(log_in))
                ))
                .service(web::resource("/")
                    .route(web::get().to(status))
                )
            

            .route("/test", web::get().to(p404))
            .route("/", web::get().to(index))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
